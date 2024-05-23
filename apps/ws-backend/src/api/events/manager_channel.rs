use crate::api::events::redis_channel::{RedisChannelCommands, RedisChannelResponse};
use crate::api::events::{EventName, SubscriptionsMap, UserChannelRx, UserId};
use anyhow::anyhow;
use std::collections::{HashMap, HashSet};
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::task::JoinHandle;

use super::user_subscription::UserPubSub;
use super::{ChannelId, UserCacheMap, UserChannelsMap};

#[derive(Debug)]
pub enum ManagerChannelCommands {
    Register(ManagerRegisterMessage),
    Subscribe(ManagerSubscribeMessage),
    Unsubscribe(ManagerUnsubscribeMessage),
    DropConnection(ManagerDropUserMessage),
}

#[derive(Debug)]
pub struct ManagerRegisterMessage {
    pub user_id: UserId,
    pub register_tx: oneshot::Sender<ManagerRegisterResponse>,
}

#[derive(Debug)]
pub struct ManagerRegisterResponse {
    pub channel_id: ChannelId,
    pub user_rx: UserChannelRx,
}

#[derive(Debug)]
pub struct ManagerSubscribeMessage {
    pub event_name: EventName,
    pub user_id: UserId,
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
    user_channels: UserChannelsMap,
    user_events: UserCacheMap,
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
        user_channels: HashMap::new(),
        user_events: HashMap::new(),
    };

    let code = async move {
        loop {
            let cmd = tokio::select! {
                Some(redis_response) = manager_state.redis_rx.recv() => handle_redis(redis_response,&mut manager_state).await,
                Some(command) = receiver.recv() => match command {
                    ManagerChannelCommands::Register(message) => handle_register(message,&mut manager_state).await,
                    ManagerChannelCommands::Subscribe(message) => handle_subscribe(message,&mut manager_state).await,
                    ManagerChannelCommands::Unsubscribe(message) => handle_unsubscribe(message,&mut manager_state).await,
                    ManagerChannelCommands::DropConnection(message) => handle_drop_user(message,&mut manager_state).await,
                },
                else => Ok::<_,anyhow::Error>(()),
            };

            if let Err(err) = cmd {
                eprintln!("{err:?}");
            }
        }
    };

    let manager_handle = tokio::spawn(code);

    (manager_handle, sender)
}

async fn handle_redis(
    response: RedisChannelResponse,
    manager_state: &mut ManagerState,
) -> Result<(), anyhow::Error> {
    let RedisChannelResponse::Event(event) = response;

    let evt_set = manager_state
        .subscriptions
        .get(&event)
        .ok_or(anyhow!("Received unknown event from redis."))?;

    for user in evt_set {
        manager_state
            .user_channels
            .get(user)
            .and_then(|UserPubSub { user_tx, .. }| user_tx.send(event.clone()).ok());
    }
    Ok(())
}

async fn handle_register(
    message: ManagerRegisterMessage,
    manager_state: &mut ManagerState,
) -> Result<(), anyhow::Error> {
    let ManagerRegisterMessage {
        user_id,
        register_tx,
    } = message;

    if let Some(user_channel) = manager_state.user_channels.get_mut(&user_id) {
        user_channel.connections += 1;
        println!("Connecting user {user_id} as {}", user_channel.connections);

        register_tx
            .send(ManagerRegisterResponse {
                channel_id: user_channel.connections,
                user_rx: user_channel.user_tx.subscribe(),
            })
            .unwrap();

        return Ok(());
    }

    println!("New connection for user {user_id}");
    let (user_tx, user_rx) = broadcast::channel(10);
    manager_state.user_channels.insert(
        user_id,
        UserPubSub {
            user_tx: user_tx,
            connections: 0,
        },
    );

    register_tx
        .send(ManagerRegisterResponse {
            channel_id: 0,
            user_rx: user_rx,
        })
        .unwrap();

    Ok(())
}

async fn handle_subscribe(
    message: ManagerSubscribeMessage,
    manager_state: &mut ManagerState,
) -> Result<(), anyhow::Error> {
    let ManagerSubscribeMessage {
        user_id: user_name,
        event_name,
    } = message;

    manager_state
        .subscriptions
        .entry(event_name.clone())
        .and_modify(|evt| {
            evt.insert(user_name.clone());
        })
        .or_insert_with(|| HashSet::from([user_name.clone()]));

    manager_state
        .user_events
        .entry(user_name)
        .and_modify(|user_channels| user_channels.push(event_name.clone()))
        .or_insert_with(|| vec![event_name.clone()]);

    manager_state
        .redis_tx
        .send(RedisChannelCommands::Sub(event_name))
        .await
        .expect("To send message to channel");

    Ok(())
}

async fn handle_unsubscribe(
    message: ManagerUnsubscribeMessage,
    manager_state: &mut ManagerState,
) -> Result<(), anyhow::Error> {
    let ManagerUnsubscribeMessage {
        event_name,
        user_id,
    } = message;

    let event = match manager_state.subscriptions.get_mut(&event_name) {
        Some(t) => t,
        None => return Ok(()),
    };

    if event.remove(&user_id) {
        println!("Unsubbing {user_id} from {event_name}.");
    }

    if event.is_empty() {
        manager_state
            .redis_tx
            .send(RedisChannelCommands::Unsub(event_name.clone()))
            .await
            .expect("To unsub from redis");
    }

    let user_events = manager_state
        .user_events
        .get_mut(&user_id)
        .expect("User didn't have event in cache");

    let idx = user_events
        .iter()
        .position(|r| *r == event_name)
        .expect("To find index in user_map");

    let _ = user_events.swap_remove(idx);

    Ok(())
}

async fn handle_drop_user(
    message: ManagerDropUserMessage,
    manager_state: &mut ManagerState,
) -> Result<(), anyhow::Error> {
    let ManagerDropUserMessage { user_id } = message;

    let user_connection = manager_state.user_channels.get_mut(&user_id).unwrap();

    if user_connection.connections > 0 {
        user_connection.connections -= 1;
        return Ok(());
    } else {
        println!("User dropped");
        manager_state.user_channels.remove(&user_id);
    }

    let user_events = match manager_state.user_events.remove(&user_id) {
        Some(u) => u,
        None => {
            println!("User had no events");
            return Ok(());
        }
    };

    if user_events.is_empty() {
        return Ok(());
    }

    for event in user_events.iter() {
        let event_map = manager_state
            .subscriptions
            .get_mut(event)
            .expect("To get event");

        event_map.remove(&user_id);

        if event_map.is_empty() {
            manager_state
                .redis_tx
                .send(RedisChannelCommands::Unsub(event.clone()))
                .await
                .expect("To unsub from redis");
        }
    }

    Ok(())
}
