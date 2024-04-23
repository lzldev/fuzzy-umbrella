use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Post {
    pub id: String,
    pub content: String,
    pub image_key: String,
    pub created_at: String,
    pub user_id: usize,
}
