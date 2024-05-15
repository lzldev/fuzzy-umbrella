use crate::api::events::redis_channel::RedisChannelCommands;
use crate::api::events::state::{EventName, SubscriptionsMap, UserId, UserSubscriptionsMap};
use crate::api::events::UserChannelSender;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

#[derive(Debug)]
pub enum ManagerChannelCommands {
    Subscribe(ManagerSubscribeMessage),
    Unsubscribe(ManagerUnsubscribeMessage),
    DropUser(ManagerDropUserMessage),
}

#[derive(Debug)]
pub struct ManagerSubscribeMessage {
    pub event_name: EventName,
    pub user_id: UserId,
    pub sender: UserChannelSender,
}

#[derive(Debug)]
pub struct ManagerUnsubscribeMessage {
    pub event_name: EventName,
    pub user_id: UserId,
}

#[derive(Debug)]
pub struct ManagerDropUserMessage {
    pub user_id: UserId,
}

#[derive(Debug)]
struct ManagerState {
    subscriptions: Arc<RwLock<SubscriptionsMap>>,
    redis_sender: Sender<RedisChannelCommands>,
    user_cache: UserSubscriptionsMap,
}

pub fn start_manager(
    subscriptions: Arc<RwLock<SubscriptionsMap>>,
    redis_sender: Sender<RedisChannelCommands>,
) -> (JoinHandle<()>, Sender<ManagerChannelCommands>) {
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<ManagerChannelCommands>(5);

    let mut manager_state = ManagerState {
        subscriptions,
        redis_sender,
        user_cache: HashMap::new(),
    };

    let code = async move {
        loop {
            tokio::select! {
                Some(command) = receiver.recv() => match command {
                    ManagerChannelCommands::Subscribe(message) => handle_subscribe(message,&mut manager_state).await,
                    ManagerChannelCommands::Unsubscribe(message) => handle_unsubscribe(message,&mut manager_state).await,
                    ManagerChannelCommands::DropUser(message) => handle_drop_user(message,&mut manager_state).await,
                }
            }
        }
    };

    let manager_handle = tokio::spawn(code);

    (manager_handle, sender)
}

async fn handle_subscribe(message: ManagerSubscribeMessage, manager_state: &mut ManagerState) {
    let ManagerSubscribeMessage {
        user_id: user_name,
        event_name,
        sender,
    } = message;

    let mut map = manager_state.subscriptions.write().await;

    if !map.contains_key(&event_name) {
        map.insert(
            event_name.clone(),
            HashMap::from([(user_name.clone(), sender)]),
        );
    } else {
        let subs = map.get_mut(&event_name).unwrap();
        subs.insert(user_name.clone(), sender);
    }
    drop(map);

    {
        let mut user_map = &mut manager_state.user_cache;
        if user_map.contains_key(&user_name) {
            user_map
                .get_mut(&user_name)
                .unwrap()
                .push(event_name.clone());
        } else {
            user_map.insert(user_name, vec![event_name.clone()]);
        }
    };

    manager_state
        .redis_sender
        .send(RedisChannelCommands::Sub(event_name))
        .await
        .expect("To send message to channel");
}

async fn handle_unsubscribe(message: ManagerUnsubscribeMessage, manager_state: &mut ManagerState) {
    let ManagerUnsubscribeMessage {
        event_name,
        user_id: user_name,
    } = message;

    let mut map = manager_state.subscriptions.write().await;
    if map.contains_key(&event_name) {
        map.remove(&event_name);
    }
    drop(map);

    let mut map = &mut manager_state.user_cache;
    let mut user_map = map.get_mut(&user_name).unwrap();
    let idx = user_map
        .binary_search(&event_name)
        .expect("To find index in user_map");
    let _ = user_map.swap_remove(idx);
    //TODO: Send UNSUB to redis if event is empty
}

async fn handle_drop_user(message: ManagerDropUserMessage, manager_state: &mut ManagerState) {
    let ManagerDropUserMessage { user_id } = message;

    let mut user_cache = &mut manager_state.user_cache;
    let user_events = match user_cache.remove(&user_id) {
        Some(u) => u,
        None => {
            dbg!("[MANAGER] Trying to drop nonexistent user");
            return;
        }
    };

    if (user_events.is_empty()) {
        return;
    }

    let mut events_map = manager_state.subscriptions.write().await;
    for event in user_events.iter() {
        let event_map = events_map.get_mut(event).expect("To get event");
        event_map
            .remove(&user_id)
            .expect("To Remove user from event.");
    }
}
