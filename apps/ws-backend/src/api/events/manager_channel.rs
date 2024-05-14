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
    Subscribe(ManagerSubscribeCommand),
    Unsubscribe(ManagerUnsubscribeMessage),
    DropUser(ManagerDropUserMessage),
}

#[derive(Debug)]
pub struct ManagerSubscribeCommand {
    pub event_name: EventName,
    pub user_name: UserId,
    pub sender: UserChannelSender,
}

#[derive(Debug)]
pub struct ManagerUnsubscribeMessage {
    pub event_name: EventName,
    pub user_name: UserId,
}

#[derive(Debug)]
pub struct ManagerDropUserMessage {
    pub event_name: EventName,
    pub user_name: UserId,
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

    let manager_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(command) = receiver.recv() => match command {
                    ManagerChannelCommands::Subscribe(message) => handle_subscribe(message,&mut manager_state).await,
                    ManagerChannelCommands::Unsubscribe(message) => handle_unsubscribe(message,&mut manager_state).await,
                    ManagerChannelCommands::DropUser(message) => handle_drop_user(message,&mut manager_state).await,
                },
            }
        }
    });

    (manager_handle, sender)
}

async fn handle_subscribe(message: ManagerSubscribeCommand, manager_state: &mut ManagerState) {}

async fn handle_unsubscribe(message: ManagerUnsubscribeMessage, manager_state: &mut ManagerState) {}

async fn handle_drop_user(message: ManagerDropUserMessage, manager_state: &mut ManagerState) {}
