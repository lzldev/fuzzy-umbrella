use crate::api::events::EventName;
use fred::clients::RedisClient;
use fred::interfaces::{EventInterface, PubsubInterface};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

#[derive(Debug)]
pub enum RedisChannelCommands {
    Sub(EventName),
    Unsub(EventName),
}

#[derive(Debug)]
pub enum RedisChannelResponse {
    Event(EventName),
}

#[derive(Debug)]
struct PubSubChannelState {
    redis_client: RedisClient,
    response_tx: mpsc::Sender<RedisChannelResponse>,
}

pub fn start_redis_task(
    redis_client: RedisClient,
) -> (
    JoinHandle<()>,
    mpsc::Sender<RedisChannelCommands>,
    mpsc::Receiver<RedisChannelResponse>,
) {
    let (response_tx, response_rx) = mpsc::channel::<RedisChannelResponse>(5);
    let (command_tx, mut command_rx) = mpsc::channel::<RedisChannelCommands>(5);

    let mut redis_rx = redis_client.message_rx();

    let redis_channel_state = PubSubChannelState {
        redis_client,
        response_tx,
    };

    let handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(command) = command_rx.recv() => handle_command(&redis_channel_state,command).await,
                Ok(redis_msg) = redis_rx.recv() => handle_redis(&redis_channel_state,redis_msg).await,
            }
        }
    });

    (handle, command_tx, response_rx)
}

async fn handle_redis(redis_channel_state: &PubSubChannelState, message: fred::types::Message) {
    let channel: Arc<str> = message.channel.to_string().into();

    redis_channel_state
        .response_tx
        .send(RedisChannelResponse::Event(channel))
        .await
        .expect("To send message to manager");
}

async fn handle_command(state: &PubSubChannelState, command: RedisChannelCommands) {
    match command {
        RedisChannelCommands::Sub(evt) => {
            state
                .redis_client
                .subscribe((*evt).to_owned())
                .await
                .expect("To subscribe to redis event");
        }
        RedisChannelCommands::Unsub(evt) => {
            state
                .redis_client
                .unsubscribe((*evt).to_owned())
                .await
                .expect("To unsubscribe to redis event");
        }
    }
}
