use super::UserChannelTx;

#[derive(Debug)]
pub struct UserPubSub {
    pub user_tx: UserChannelTx,
    pub connections: usize,
}
