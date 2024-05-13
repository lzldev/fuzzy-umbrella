use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use fred::{
    clients::RedisClient,
    interfaces::{ClientLike, EventInterface, PubsubInterface},
    types::RedisConfig,
};
use tokio::{
    sync::{mpsc, RwLock},
    task::JoinHandle,
};

use super::{
    redis_channel::RedisChannelCommands, user_subscription::UserSubscription, UserChannelSender,
};

type EventName = Arc<str>;
type UserId = Arc<str>;

#[derive(Debug)]
pub struct EventChannelState {
    _manager_task_handle: JoinHandle<()>,
    manager_sender: mpsc::Sender<RedisChannelCommands>,
    user_events: Mutex<HashMap<UserId, Vec<EventName>>>,
    subscriptions: Arc<RwLock<HashMap<EventName, HashMap<UserId, UserChannelSender>>>>,
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

        let subscriptions = Arc::new(RwLock::new(HashMap::new()));
        let subscriptions_handle = subscriptions.clone();

        redis_client.init().await.expect("To connect to redis");
        let (tx, mut rx) = tokio::sync::mpsc::channel::<RedisChannelCommands>(5);

        let manager_task = tokio::spawn(async move {
            let subscriptions = subscriptions_handle;
            let redis_client = redis_client;
            let mut redis_channel = redis_client.message_rx();

            let handle_redis = |message: fred::types::Message| async {
                dbg!(message);
            };

            let handle_channel = |command: RedisChannelCommands| async {
                match command {
                    RedisChannelCommands::Sub(evt) => {
                        redis_client
                            .subscribe((*evt).to_owned())
                            .await
                            .expect("To subscribe to redis event");
                    }
                    RedisChannelCommands::Unsub(evt) => {
                        redis_client
                            .unsubscribe((*evt).to_owned())
                            .await
                            .expect("To unsubscribe to redis event");
                    }
                }
            };

            loop {
                tokio::select! {
                    Some(msg) = rx.recv() => handle_channel(msg).await,
                    Ok(redis_msg) = redis_channel.recv() => handle_redis(redis_msg).await,
                };
            }
        });

        Self {
            user_events: Mutex::new(HashMap::new()),
            _manager_task_handle: manager_task,
            manager_sender: tx,
            subscriptions,
        }
    }

    pub async fn subscribe(&self, event: Arc<str>, user: Arc<str>, sender: UserChannelSender) {
        let mut map = self.subscriptions.write().await;

        if map.contains_key(&event) {
            let subs = map.get_mut(&event).unwrap();
            subs.insert(user.clone(), sender);
            drop(map);

            let mut user_map = self.user_events.lock().unwrap();
            user_map.get_mut(&user).unwrap().push(event);
            return;
        }

        map.insert(event.clone(), HashMap::from([(user, sender)]));

        drop(map);

        self.manager_sender
            .send(RedisChannelCommands::Sub(event))
            .await
            .expect("To send message to channel");
    }

    #[allow(dead_code, unused_variables)]
    pub fn unsubscribe(&mut self, event: Arc<str>, user: Arc<str>) {}

    #[allow(dead_code, unused_variables)]
    pub fn drop_user(&mut self, user: Arc<str>) {}
}

/***
 * Unsub users should kill every subscription from user
 *
 * When unsubing someone from an event:
 *      Check if event is empty.
*           if so -> stop subing for it in redis task.
*

For achieving both i should:
    keep a user -> Vec<Events> (could be a hashset)
        for a faster user_drop


Dropping:
    Iterate through user subscribed events:
        For every event remove user subscription from set:
            problem:
                since im removing from a set.
                    i have to create a new UserSubscription matching the
                    user's hash to drop it ?
 */
