use std::{collections::HashMap, sync::Arc};

use fred::{
    clients::RedisClient,
    interfaces::{ClientLike, EventInterface, PubsubInterface},
    types::RedisConfig,
};

use tokio::{
    sync::{
        mpsc::{self, Sender},
        Mutex, RwLock,
    },
    task::JoinHandle,
};

use super::{redis_channel::RedisChannelCommands, UserChannelSender};

type EventName = Arc<str>;
type UserId = Arc<str>;

#[derive(Debug)]
pub struct EventChannelState {
    _manager_task_handle: JoinHandle<()>,
    manager_sender: mpsc::Sender<RedisChannelCommands>,
    user_events: Mutex<HashMap<UserId, Vec<EventName>>>, // This can be a Box::pin?
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

            async fn handle_redis(
                message: fred::types::Message,
                subscriptions: &Arc<RwLock<HashMap<Arc<str>, HashMap<Arc<str>, Sender<Arc<str>>>>>>,
            ) {
                let channel: Arc<str> = message.channel.to_string().into();
                let map = subscriptions.write().await;
                if !map.contains_key(&channel) {
                    eprintln!("Subscribed event not found in map {:?}", &channel);
                    return;
                }

                let channels = map.get(&channel).unwrap().values();
                let mut counter = 0;
                for c in channels {
                    c.send(channel.clone()).await.unwrap();
                    counter = counter + 1;
                }

                dbg!(counter);
            }

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

            // let handle_channel = Box::pin(handle_channel);

            loop {
                tokio::select! {
                    Some(msg) = rx.recv() => handle_channel(msg).await,
                    Ok(redis_msg) = redis_channel.recv() => handle_redis(redis_msg,&subscriptions).await,
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

        self.manager_sender
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
