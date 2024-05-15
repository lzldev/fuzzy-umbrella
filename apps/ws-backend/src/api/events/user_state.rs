use crate::api::events::manager_channel::ManagerChannelCommands;
use crate::api::events::state::UserId;
use crate::api::events::UserChannelCommand;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct UserState {
    pub user_id: UserId,
    pub user_sender: mpsc::Sender<UserChannelCommand>,
    pub manager_sender: mpsc::Sender<ManagerChannelCommands>,
}
