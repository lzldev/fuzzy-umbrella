use std::time::Duration;
use std::{collections::HashMap, sync::Arc};

use fred::{
    clients::RedisClient,
    interfaces::{ClientLike, EventInterface, PubsubInterface},
    types::RedisConfig,
};

use crate::api::events::manager_channel::start_manager;
use crate::api::events::redis_channel::start_redis_task;
use tokio::{
    sync::{
        mpsc::{self, Sender},
        Mutex, RwLock,
    },
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
    redis_sender: mpsc::Sender<RedisChannelCommands>,
    user_events: Mutex<UserSubscriptionsMap>, // This can be a Box::pin?
    subscriptions: Arc<RwLock<SubscriptionsMap>>, // This lock can be removed by doing | Redis -> Manager
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

        redis_client.init().await.expect("To connect to redis");

        let (_redis_handle, redis_sender) = start_redis_task(redis_client, subscriptions_handle);

        let (_manager_handle, manager_sender) =
            start_manager(subscriptions.clone(), redis_sender.clone());

        let _manager_handle = tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(10));
                eprintln!("Waa");
            }
        });

        Self {
            _redis_handle,
            _manager_handle,
            redis_sender,
            subscriptions,
            user_events: Mutex::new(HashMap::new()),
        }
    }

    pub async fn subscribe(&self, event: Arc<str>, user: Arc<str>, sender: UserChannelSender) {
        let mut map = self.subscriptions.write().await;

        if !map.contains_key(&event) {
            map.insert(event.clone(), HashMap::from([(user.clone(), sender)]));
        } else {
            let subs = map.get_mut(&event).unwrap();
            subs.insert(user.clone(), sender);
        }
        drop(map);

        let mut user_map = self.user_events.lock().await;
        if user_map.contains_key(&user) {
            user_map.get_mut(&user).unwrap().push(event.clone());
        } else {
            user_map.insert(user, vec![event.clone()]);
        }
        drop(user_map);

        self.redis_sender
            .send(RedisChannelCommands::Sub(event))
            .await
            .expect("To send message to channel");
    }

    #[allow(dead_code, unused_variables)]
    pub async fn unsubscribe(&self, event: EventName, user: UserId) {
        let mut map = self.subscriptions.write().await;
        if map.contains_key(&event) {
            map.remove(&event);
        }
        drop(map);

        let mut map = self.user_events.lock().await;
        let mut user_map = map.get_mut(&user).unwrap();
        let idx = user_map
            .binary_search(&event)
            .expect("To find index in user_map");
        let _ = user_map.swap_remove(idx);
        //TODO: Send UNSUB to redis if event is empty
    }

    #[allow(dead_code, unused_variables)]
    pub async fn drop_user(&self, user: UserId) {
        let mut user_map = self.user_events.lock().await;
        let user_events = user_map.get(&user).expect("To find user");

        if (user_events.is_empty()) {
            return;
        }

        let mut events_map = self.subscriptions.write().await;
        for event in user_events.iter() {
            let event_map = events_map.get_mut(event).expect("To get event");
            event_map.remove(&user).expect("To Remove user from event.");
        }
    }
}
