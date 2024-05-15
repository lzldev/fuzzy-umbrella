use crate::api::events::manager_channel::{
    ManagerChannelCommands, ManagerDropUserMessage, ManagerRegisterMessage,
    ManagerRegisterResponse, ManagerSubscribeMessage, ManagerUnsubscribeMessage,
};
use crate::api::events::user_state::UserState;
use crate::api::events::UserChannelCommand;
use artspace_shared::client::ClientMessage;
use artspace_shared::server::{ErrorMessage, ReceivedMessage, ServerMessage};
use rocket::futures::{SinkExt, StreamExt};
use rocket_ws::{stream::DuplexStream, Message};
use tokio::sync::mpsc;

use super::UserId;

pub async fn start_user_channel(
    mut stream: DuplexStream,
    user_id: UserId,
    manager_sender: mpsc::Sender<ManagerChannelCommands>,
) -> Result<(), rocket_ws::result::Error> {
    let (register_tx, register_rx) = tokio::sync::oneshot::channel::<ManagerRegisterResponse>();

    manager_sender
        .send(ManagerChannelCommands::Register(ManagerRegisterMessage {
            user_id: user_id.clone(),
            register_tx,
        }))
        .await
        .unwrap();

    let ManagerRegisterResponse {
        channel_id,
        mut user_rx,
    } = register_rx.await.unwrap();

    let user_state = UserState {
        channel_id,
        user_id,
        manager_sender,
    };

    loop {
        tokio::select! {
            soc = stream.next() => {
                let message = match soc {
                    Some(Ok(message)) => message,
                    _ => break
                };

                handle_message(&mut stream,&user_state,message).await
            },
            Ok(message) = user_rx.recv() => handle_response(&mut stream,&user_state,message).await,
            else => break
        }
    }

    dbg!("Closing Socket");

    if user_state.manager_sender.is_closed() {
        return Ok(());
    }

    drop(user_rx);

    user_state
        .manager_sender
        .send(ManagerChannelCommands::DropConnection(
            ManagerDropUserMessage {
                user_id: user_state.user_id,
            },
        ))
        .await
        .unwrap();

    Ok(())
}

async fn handle_response(
    stream: &mut DuplexStream,
    _state: &UserState,
    message: UserChannelCommand,
) {
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
        #[allow(unreachable_patterns)]
        _ => {
            let error = ServerMessage::Error(ErrorMessage {
                message: String::from("Message not implemented yet."),
                cause: "".into(),
            });
            let error = serde_json::to_string(&error).unwrap();

            let _ = stream.send(error.into()).await;
        }
    };
}
