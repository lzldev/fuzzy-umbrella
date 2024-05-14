mod redis_channel;
mod state;
mod user_subscription;

use std::sync::Arc;

use artspace_shared::{
    client::ClientMessage,
    server::{ErrorMessage, ReceivedMessage, ServerMessage},
};
use rocket::futures::{SinkExt, StreamExt};
use rocket::{Build, Rocket, State};
use rocket_ws::{stream::DuplexStream, Message};
use tokio::sync::mpsc::Sender;
use ws_backend::auth::ClerkUser;

use self::state::EventChannelState;

pub type UserChannelValue = Arc<str>; // User Socket will only receive the event name for now . no value.
pub type UserChannelSender = tokio::sync::mpsc::Sender<UserChannelValue>;

pub async fn register(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .manage(EventChannelState::create().await)
        .mount("/ws/", routes![sub_events])
}

#[get("/sub")]
fn sub_events<'a>(
    ws: rocket_ws::WebSocket,
    user: ClerkUser<'_>,
    state: &'a State<EventChannelState>,
) -> rocket_ws::Channel<'a> {
    let (sender, receiver) = tokio::sync::mpsc::channel::<UserChannelValue>(5);
    let user_id: Arc<str> = user.token.claims.sub.clone().into();

    ws.channel(move |mut stream| {
        Box::pin(async move {
            let mut receiver = receiver;

            loop {
                tokio::select! {
                    r = stream.next() => {
                        let message = match r {
                            Some(r) => r,
                            None => break,
                        };

                        handle_message(&mut stream, state, &user_id, &sender, message?).await;
                    },
                    r = receiver.recv() => {
                        let message = match r {
                            Some(m) => m,
                            None => break,
                        };

                        handle_redis(&mut stream, state, &user_id, &sender, message).await;
                    }
                }
            }

            Ok(())
        })
    })
}

async fn handle_redis(
    stream: &mut DuplexStream,
    _state: &State<EventChannelState>,
    _user_id: &Arc<str>,
    _sender: &Sender<Arc<str>>,
    event: Arc<str>,
) {
    let msg = ServerMessage::Received(ReceivedMessage {
        event_name: (*event).to_owned(),
    });

    stream
        .send(serde_json::to_string(&msg).unwrap().into())
        .await
        .unwrap();
}

async fn handle_message(
    stream: &mut DuplexStream,
    state: &State<EventChannelState>,
    user_id: &Arc<str>,
    sender: &UserChannelSender,
    message: Message,
) {
    if !message.is_text() {
        return;
    }

    let txt = message.to_text().unwrap();

    if txt == "/debug" {
        let dbg_msg = format!("{:#?}", &state);
        eprintln!("{dbg_msg}");
        stream.send(dbg_msg.into()).await.unwrap();
        return;
    }

    let message = match serde_json::from_str::<ClientMessage>(txt) {
        Ok(m) => m,
        Err(err) => {
            let error = ServerMessage::Error(ErrorMessage {
                message: String::from("Invalid Message"),
                cause: err.to_string(),
            });

            let error = serde_json::to_string(&error).unwrap();

            let _ = stream.send(error.into()).await;
            return;
        }
    };

    match message {
        ClientMessage::Subscribe(msg) => {
            state
                .subscribe(msg.event_name.into(), user_id.clone(), sender.clone())
                .await;
        }
        ClientMessage::Unsubscribe(msg) => {
            state
                .unsubscribe(msg.event_name.into(), user_id.clone())
                .await;
        }
        _ => {
            let error = ServerMessage::Error(ErrorMessage {
                message: String::from("Message not implemented yet."),
                cause: "".into(),
            });
            let error = serde_json::to_string(&error).unwrap();

            let _ = stream.send(error.into()).await;
            return;
        }
    };
}
