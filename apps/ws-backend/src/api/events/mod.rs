mod redis_channel;
mod state;
mod user_subscription;

use std::sync::Arc;

use artspace_shared::{
    client::ClientMessage,
    server::{ErrorMessage, ServerMessage},
};
use rocket::{Build, Rocket, State};
use ws_backend::auth::ClerkUser;

use crate::api::events::user_subscription::UserSubscription;

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
    user: ClerkUser<'a>,
    state: &'a State<EventChannelState>,
) -> rocket_ws::Channel<'a> {
    use rocket::futures::{SinkExt, StreamExt};

    let (sender, receiver) = tokio::sync::mpsc::channel::<UserChannelValue>(5);
    let user_id: Arc<str> = user.token.claims.sub.clone().into();

    let _ex_sub = UserSubscription {
        user: user_id.clone(),
        sender: sender.clone(),
    };

    ws.channel(move |mut stream| {
        Box::pin(async move {
            let mut _receiver = receiver;
            loop {
                let msg = match stream.next().await {
                    Some(m) => m?, // This will return an error from the Async block which will skip unsubbing from state.
                    None => break,
                };

                if !msg.is_text() {
                    continue;
                }

                let txt = msg.to_text().unwrap();

                if txt == "/debug" {
                    let dbg_msg = format!("{:#?}", &state);
                    eprintln!("{dbg_msg}");
                    stream.send(dbg_msg.into()).await.unwrap();
                    continue;
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
                        continue;
                    }
                };

                let subscribe_message = match message {
                    ClientMessage::Subscribe(msg) => msg,
                    _ => {
                        let error = ServerMessage::Error(ErrorMessage {
                            message: String::from("Message not implemented yet."),
                            cause: "".into(),
                        });
                        let error = serde_json::to_string(&error).unwrap();

                        let _ = stream.send(error.into()).await;
                        continue;
                    }
                };

                let _ = &state
                    .subscribe(
                        subscribe_message.event_name.into(),
                        user_id.clone(),
                        sender.clone(),
                    )
                    .await;
            }

            Ok(())
        })
    })
}
