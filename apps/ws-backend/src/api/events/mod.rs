mod manager_channel;
mod redis_channel;
mod state;
mod user_state;
mod user_subscription;

use std::sync::Arc;

use crate::api::events::state::UserId;
use crate::api::events::user_state::UserState;
use artspace_shared::{
    client::ClientMessage,
    server::{ErrorMessage, ReceivedMessage, ServerMessage},
};
use rocket::futures::{SinkExt, StreamExt};
use rocket::{Build, Rocket, State};
use rocket_ws::{stream::DuplexStream, Message};
use tokio::sync::mpsc;

use crate::api::events::manager_channel::{
    ManagerChannelCommands, ManagerDropUserMessage, ManagerSubscribeMessage,
    ManagerUnsubscribeMessage,
};
use ws_backend::auth::ClerkUser;

use self::state::EventChannelState;

pub type UserChannelCommand = Arc<str>; // User Socket will only receive the event name for now . no value.
pub type UserChannelSender = mpsc::Sender<UserChannelCommand>;

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
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<UserChannelCommand>(5);

    let user_state = UserState {
        user_id: user.token.claims.sub.clone().into(),
        user_sender: sender,
        manager_sender: state.manager_sender.clone(),
    };

    let handler = |mut stream: DuplexStream| async move {
        loop {
            tokio::select! {
                r = stream.next() => {
                    let message = match r {
                        Some(Ok(message)) => message,
                        _ => break,
                    };

                    handle_message(&mut stream,&user_state,message).await;
                },
                r = receiver.recv() => {
                    let message = match r {
                        Some(m) => m,
                        None => break,
                    };

                    handle_redis(&mut stream,&user_state,message).await;
                }
            }
        }

        user_state
            .manager_sender
            .send(ManagerChannelCommands::DropUser(ManagerDropUserMessage {
                user_id: user_state.user_id,
            }))
            .await
            .unwrap();

        Ok(())
    };
    ws.channel(move |mut stream| Box::pin(handler(stream)))
}

async fn handle_redis(stream: &mut DuplexStream, state: &UserState, message: UserChannelCommand) {
    let msg = ServerMessage::Received(ReceivedMessage {
        event_name: (*message).to_owned(),
    });

    stream
        .send(serde_json::to_string(&msg).unwrap().into())
        .await
        .unwrap();
}

async fn handle_message(stream: &mut DuplexStream, user_state: &UserState, message: Message) {
    let txt = if message.is_text() {
        message.to_text().unwrap()
    } else {
        return;
    };

    let message = match serde_json::from_str::<ClientMessage>(txt) {
        Ok(m) => m,
        Err(err) => {
            let error = serde_json::to_string(&ServerMessage::Error(ErrorMessage {
                message: String::from("Invalid Message"),
                cause: err.to_string(),
            }))
            .unwrap();

            let _ = stream.send(error.into()).await;
            return;
        }
    };

    match message {
        ClientMessage::Subscribe(msg) => {
            user_state
                .manager_sender
                .send(ManagerChannelCommands::Subscribe(ManagerSubscribeMessage {
                    user_id: user_state.user_id.clone(),
                    sender: user_state.user_sender.clone(),
                    event_name: msg.event_name.into(),
                }))
                .await
                .unwrap();
        }
        ClientMessage::Unsubscribe(msg) => {
            user_state
                .manager_sender
                .send(ManagerChannelCommands::Unsubscribe(
                    ManagerUnsubscribeMessage {
                        user_id: user_state.user_id.clone(),
                        event_name: msg.event_name.into(),
                    },
                ))
                .await
                .unwrap();
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
