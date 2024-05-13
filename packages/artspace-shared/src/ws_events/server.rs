use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase", tag = "event", content = "message")]
#[ts(export)]
pub enum ServerMessage {
    Received(ReceivedMessage),
    Error(ErrorMessage),
    Close,
}

#[derive(Debug, Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReceivedMessage {
    pub event_name: String,
}

#[derive(Debug, Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ErrorMessage {
    pub message: String,
    pub cause: String,
}
