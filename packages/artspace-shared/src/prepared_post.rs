use redis_macros::FromRedisValue;
use serde::{Deserialize, Serialize};
use structmap::{FromMap, ToMap};
use structmap_derive::{FromMap, ToMap};
use ts_rs::TS;

#[derive(Default, Debug, Serialize, Deserialize, TS, ToMap, FromMap, FromRedisValue, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PreparedPost {
    pub id: String,
    pub content: String,
    pub user_id: String,
}
