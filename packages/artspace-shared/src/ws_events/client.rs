use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase", tag = "event", content = "message")]
#[ts(export)]
pub enum ClientMessage {
    Subscribe(SubscribeMessage),
    Unsubscribe(UnsubscribeMessage),
}

#[derive(Debug, Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct SubscribeMessage {
    pub event_name: String,
}

#[derive(Debug, Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct UnsubscribeMessage {
    pub event_name: String,
}
