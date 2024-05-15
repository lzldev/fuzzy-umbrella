use crate::api::events::manager_channel::ManagerChannelCommands;
use crate::api::events::UserId;
use tokio::sync::mpsc;

use super::{ChannelId, UserChannelTx};

#[derive(Debug)]
pub struct UserState {
    pub channel_id: ChannelId,
    pub user_id: UserId,
    pub manager_sender: mpsc::Sender<ManagerChannelCommands>,
}
