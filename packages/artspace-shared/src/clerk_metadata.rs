use redis_macros::FromRedisValue;
use serde::{Deserialize, Serialize};
use structmap::ToMap;
use structmap_derive::ToMap;
use ts_rs::TS;

#[derive(Default, Debug, Serialize, Deserialize, TS, ToMap, FromRedisValue, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ClerkPrivateMetadata {
    pub user_id: usize,
}
