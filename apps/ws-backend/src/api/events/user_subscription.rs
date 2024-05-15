use super::UserChannelTx;

#[derive(Debug)]
pub struct UserPubSub {
    pub connections: usize,
    pub user_tx: UserChannelTx,
}
