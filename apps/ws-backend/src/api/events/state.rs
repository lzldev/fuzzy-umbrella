use std::{collections::HashMap, sync::Arc};

use fred::{clients::RedisClient, interfaces::ClientLike, types::RedisConfig};

use crate::api::events::manager_channel::{start_manager, ManagerChannelCommands};
use crate::api::events::redis_channel::start_redis_task;
use tokio::{
    sync::{mpsc, RwLock},
    task::JoinHandle,
};

use super::{redis_channel::RedisChannelCommands, UserChannelSender};

pub type EventName = Arc<str>;
pub type UserId = Arc<str>;
pub type SubscriptionsMap = HashMap<EventName, HashMap<UserId, UserChannelSender>>;
pub type UserSubscriptionsMap = HashMap<UserId, Vec<EventName>>;

#[derive(Debug)]
pub struct EventChannelState {
    _redis_handle: JoinHandle<()>,
    _manager_handle: JoinHandle<()>,
    pub redis_sender: mpsc::Sender<RedisChannelCommands>,
    pub manager_sender: mpsc::Sender<ManagerChannelCommands>,
}

impl EventChannelState {
    pub async fn create() -> Self {
        let subscriptions = Arc::new(RwLock::new(HashMap::new()));
        let subscriptions_handle = subscriptions.clone();

        let redis_client = RedisClient::new(
            RedisConfig {
                version: fred::types::RespVersion::RESP3,
                ..Default::default()
            },
            None,
            None,
            None,
        );

        redis_client
            .init()
            .await
            .expect("Couldn't init Redis Connection");

        let (_redis_handle, redis_sender) = start_redis_task(redis_client, subscriptions_handle);

        let (_manager_handle, manager_sender) =
            start_manager(subscriptions.clone(), redis_sender.clone());

        Self {
            _redis_handle,
            _manager_handle,
            redis_sender,
            manager_sender,
        }
    }
}
