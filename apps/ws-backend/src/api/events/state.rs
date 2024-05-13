use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use artspace_shared::client::ClientMessage;
use fred::{
    clients::RedisClient,
    interfaces::{ClientLike, EventInterface},
    types::RedisConfig,
};
use rocket::{Build, Rocket, State};
use tokio::task::JoinHandle;
use ws_backend::auth::ClerkUser;

use super::{user_subscription::UserSubscription, UserChannelSender};

#[derive(Debug)]
pub struct EventChannelState {
    _redis_task_handle: JoinHandle<()>,
    pub subscriptions: HashMap<Arc<str>, HashSet<UserSubscription>>,
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

        redis_client.init().await.expect("To connect to redis");

        let redis_task_handle = tokio::spawn(async move {
            let redis_client = redis_client;
            let mut msg_channel = redis_client.message_rx();
            let _ = msg_channel.recv().await; // TODO:Implement
        });

        Self {
            _redis_task_handle: redis_task_handle,
            subscriptions: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, event: Arc<str>, user: Arc<str>, sender: UserChannelSender) {
        if self.subscriptions.contains_key(&event) {
            let subs = self.subscriptions.get_mut(&event).unwrap();
            subs.insert(UserSubscription { user, sender });
        } else {
            self.subscriptions
                .insert(event, HashSet::from([UserSubscription { user, sender }]));
        }
    }
}
