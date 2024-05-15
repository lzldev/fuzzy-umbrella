use std::sync::Arc;

use rocket::{Build, Rocket, State};
use tokio::sync::mpsc;

use ws_backend::auth::ClerkUser;

use crate::api::events::user_channel::start_user_channel;
use crate::api::events::user_state::UserState;

use self::state::EventChannelState;

mod manager_channel;
mod redis_channel;
mod state;
mod user_channel;
mod user_state;
mod user_subscription;

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
    let (sender, receiver) = tokio::sync::mpsc::channel::<UserChannelCommand>(5);

    let user_state = UserState {
        user_id: user.token.claims.sub.clone().into(),
        user_sender: sender,
        manager_sender: state.manager_sender.clone(),
    };

    ws.channel(move |stream| Box::pin(start_user_channel(stream, receiver, user_state)))
}
