use crate::api::events::state::SubscriptionsMap;
use fred::clients::RedisClient;
use fred::interfaces::{EventInterface, PubsubInterface};
use std::sync::Arc;
use tokio::{sync::RwLock, task::JoinHandle};

#[derive(Debug)]
pub enum RedisChannelCommands {
    Sub(Arc<str>),
    Unsub(Arc<str>),
}

pub fn start_redis_task(
    redis_client: RedisClient,
    subscriptions: Arc<RwLock<SubscriptionsMap>>,
) -> (
    JoinHandle<()>,
    tokio::sync::mpsc::Sender<RedisChannelCommands>,
) {
    let (sender, mut receive) = tokio::sync::mpsc::channel::<RedisChannelCommands>(5);

    let handle = tokio::spawn(async move {
        let mut redis_channel = redis_client.message_rx();

        loop {
            tokio::select! {
                Ok(redis_msg) = redis_channel.recv() => handle_redis(redis_msg,&subscriptions).await,
                Some(command) = receive.recv() => handle_channel(&redis_client,command).await,
            }
        }
    });

    (handle, sender)
}

async fn handle_redis(
    message: fred::types::Message,
    subscriptions: &Arc<RwLock<SubscriptionsMap>>,
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
        counter += 1;
    }

    println!("Sent {channel} event to {counter} connections.");
}

async fn handle_channel(redis_client: &RedisClient, command: RedisChannelCommands) {
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
}
