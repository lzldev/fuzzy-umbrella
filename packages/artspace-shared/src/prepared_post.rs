use serde::{Deserialize, Serialize};
use structmap::{FromMap, ToMap};
use structmap_derive::{FromMap, ToMap};
use ts_rs::TS;

#[derive(Default, Serialize, Deserialize, TS, ToMap, FromMap)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PreparedPost {
    pub id: String,
    pub content: String,
    pub user_id: String,
}
