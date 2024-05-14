use std::sync::Arc;

#[derive(Debug)]
pub enum RedisChannelCommands {
    Sub(Arc<str>),
    Unsub(Arc<str>),
}

struct RedisManager{
    
};

impl RedisManager {
    
};