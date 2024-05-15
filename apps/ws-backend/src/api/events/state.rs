use fred::{clients::RedisClient, interfaces::ClientLike, types::RedisConfig};

use crate::api::events::manager_channel::{start_manager_task, ManagerChannelCommands};
use crate::api::events::redis_channel::start_redis_task;
use tokio::{sync::mpsc, task::JoinHandle};

#[derive(Debug)]
pub struct EventChannelState {
    _redis_handle: JoinHandle<()>,
    _manager_handle: JoinHandle<()>,
    pub manager_tx: mpsc::Sender<ManagerChannelCommands>,
}

impl EventChannelState {
    pub async fn create() -> Self {
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

        let (_redis_handle, redis_tx, redis_rx) = start_redis_task(redis_client);
        let (_manager_handle, manager_tx) = start_manager_task(redis_tx.clone(), redis_rx);

        Self {
            _redis_handle,
            _manager_handle,
            manager_tx,
        }
    }
}
