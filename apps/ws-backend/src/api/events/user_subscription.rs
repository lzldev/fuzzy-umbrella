use std::sync::Arc;

use super::UserChannelSender;

#[derive(Debug)]
pub struct UserSubscription {
    pub user: Arc<str>,
    pub sender: UserChannelSender, //TODO: In the future instead of the event name "Arc<str>" it should be a enum like EventData
}

impl PartialEq for UserSubscription {
    fn eq(&self, other: &Self) -> bool {
        self.user == other.user
    }
}

impl Eq for UserSubscription {}

impl std::hash::Hash for UserSubscription {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.user.hash(state);
    }
}
