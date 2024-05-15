use crate::api::events::redis_channel::{RedisChannelCommands, RedisChannelResponse};
use crate::api::events::{
    EventName, SubscriptionsMap, UserChannelCommand, UserChannelSender, UserId,
    UserSubscriptionsMap,
};
use rocket::futures::future::join_all;
use rocket::futures::stream::FuturesUnordered;
use rocket::futures::StreamExt;
use std::collections::HashMap;
use tokio::sync::mpsc;
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
    user_cache: UserSubscriptionsMap,
    subscriptions: SubscriptionsMap,
    redis_tx: mpsc::Sender<RedisChannelCommands>,
    redis_rx: mpsc::Receiver<RedisChannelResponse>,
}

pub fn start_manager_task(
    redis_tx: mpsc::Sender<RedisChannelCommands>,
    redis_rx: mpsc::Receiver<RedisChannelResponse>,
) -> (JoinHandle<()>, mpsc::Sender<ManagerChannelCommands>) {
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<ManagerChannelCommands>(5);

    let mut manager_state = ManagerState {
        redis_tx,
        redis_rx,
        subscriptions: HashMap::new(),
        user_cache: HashMap::new(),
    };

    let code = async move {
        loop {
            tokio::select! {
                Some(redis_response) = manager_state.redis_rx.recv() => handle_redis(redis_response,&mut manager_state).await,
                Some(command) = receiver.recv() => match command {
                    ManagerChannelCommands::Subscribe(message) => handle_subscribe(message,&mut manager_state).await,
                    ManagerChannelCommands::Unsubscribe(message) => handle_unsubscribe(message,&mut manager_state).await,
                    ManagerChannelCommands::DropUser(message) => handle_drop_user(message,&mut manager_state).await,
                },
            }
        }
    };

    let manager_handle = tokio::spawn(code);

    (manager_handle, sender)
}

async fn handle_redis(response: RedisChannelResponse, manager_state: &mut ManagerState) {
    let RedisChannelResponse::Event(event) = response;

    let evt_map = if let Some(evt_map) = manager_state.subscriptions.get(&event) {
        evt_map
    } else {
        return;
    };

    let futs = evt_map.iter().map(|(_, sender)| {
        let event = event.clone();
        sender.send(event)
    });

    join_all(futs).await;
}

async fn handle_subscribe(message: ManagerSubscribeMessage, manager_state: &mut ManagerState) {
    let ManagerSubscribeMessage {
        user_id: user_name,
        event_name,
        sender,
    } = message;

    let subscriptions = &mut manager_state.subscriptions;

    subscriptions
        .entry(event_name.clone())
        .and_modify(|m| {
            m.insert(user_name.clone(), sender.clone());
        })
        .or_insert_with(|| HashMap::from([(user_name.clone(), sender)]));

    let user_cache = &mut manager_state.user_cache;

    user_cache
        .entry(user_name)
        .or_insert_with(|| vec![event_name.clone()]);

    manager_state
        .redis_tx
        .send(RedisChannelCommands::Sub(event_name))
        .await
        .expect("To send message to channel");
}

async fn handle_unsubscribe(message: ManagerUnsubscribeMessage, manager_state: &mut ManagerState) {
    let ManagerUnsubscribeMessage {
        event_name,
        user_id: user_name,
    } = message;

    let map = &mut manager_state.subscriptions;
    if map.contains_key(&event_name) {
        map.remove(&event_name);
    }

    let map = &mut manager_state.user_cache;
    let user_map = map.get_mut(&user_name).unwrap();
    let idx = user_map
        .binary_search(&event_name)
        .expect("To find index in user_map");
    let _ = user_map.swap_remove(idx);
    //TODO: Send UNSUB to redis if event is empty
}

async fn handle_drop_user(message: ManagerDropUserMessage, manager_state: &mut ManagerState) {
    let ManagerDropUserMessage { user_id } = message;

    let user_cache = &mut manager_state.user_cache;
    let user_events = match user_cache.remove(&user_id) {
        Some(u) => u,
        None => {
            dbg!("[MANAGER] Trying to drop nonexistent user");
            return;
        }
    };

    if user_events.is_empty() {
        return;
    }

    let events_map = &mut manager_state.subscriptions;
    for event in user_events.iter() {
        let event_map = events_map.get_mut(event).expect("To get event");
        event_map
            .remove(&user_id)
            .expect("To Remove user from event.");
    }
}
