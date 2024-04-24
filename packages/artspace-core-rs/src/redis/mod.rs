mod artspace_redis_commands;

pub mod hash_map;
pub mod keys;
pub use artspace_redis_commands::*;

use redis::{aio::MultiplexedConnection, AsyncCommands, RedisError, ToRedisArgs};

pub async fn key_exists<K>(con: &mut MultiplexedConnection, key: K) -> Result<bool, RedisError>
where
    K: ToRedisArgs + Send + Sync,
{
    let len: usize = con.hlen(key).await?;

    if len == 0 {
        return Ok(false);
    }

    return Ok(false);
}
