mod state;
mod user_subscription;

use std::sync::Arc;

use rocket::{Build, Rocket, State};
use ws_backend::auth::ClerkUser;

use self::state::EventChannelState;

pub type UserChannelSender = tokio::sync::mpsc::Sender<Arc<str>>;

pub async fn register(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .manage(EventChannelState::create().await)
        .mount("/ws/", routes![sub_events])
}

#[get("/sub")]
fn sub_events<'a>(
    ws: rocket_ws::WebSocket,
    user: ClerkUser<'a>,
    state: &State<EventChannelState>,
) -> rocket_ws::Channel<'a> {
    use rocket::futures::{SinkExt, StreamExt};

    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                let msg = match stream.next().await {
                    Some(m) => m?, // this breaks but doesn't unsub from State
                    None => break,
                };

                let _ = stream.send(msg).await;
            }

            Ok(())
        })
    })
}
