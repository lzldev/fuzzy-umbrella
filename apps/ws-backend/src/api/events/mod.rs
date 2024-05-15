use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use rocket::{Build, Rocket, State};
use tokio::sync::broadcast;

use ws_backend::auth::ClerkUser;

use crate::api::events::user_channel::start_user_channel;

use self::state::EventChannelState;
use self::user_subscription::UserPubSub;

mod manager_channel;
mod redis_channel;
mod state;
mod user_channel;
mod user_state;
mod user_subscription;

pub type ChannelId = usize;
pub type EventName = Arc<str>;
pub type UserId = Arc<str>;

pub type SubscriptionsMap = HashMap<EventName, HashSet<UserId>>;

pub type UserChannelCommand = Arc<str>; // User Socket will only receive the event name for now . no value.
                                        //
pub type UserChannelTx = broadcast::Sender<UserChannelCommand>;
pub type UserChannelRx = broadcast::Receiver<UserChannelCommand>;

pub type UserCacheMap = HashMap<UserId, Vec<EventName>>;
pub type UserChannelsMap = HashMap<UserId, UserPubSub>;

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
    ws.channel(move |stream| {
        Box::pin(start_user_channel(
            stream,
            user.token.claims.sub.clone().into(),
            state.manager_tx.clone(),
        ))
    })
}
