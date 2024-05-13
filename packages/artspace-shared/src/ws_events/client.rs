use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum ClientMessage {
    Subscribe(SubscribeMessage),
}

#[derive(Debug, Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct SubscribeMessage {
    pub event_name: String,
}
