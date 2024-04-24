use redis::{aio::MultiplexedConnection, AsyncCommands, RedisError, ToRedisArgs};
use structmap::ToMap;

use crate::into_tuple_vec::IntoTupleVec;

pub async fn clear_hmap<K>(con: &mut MultiplexedConnection, key: K) -> Result<usize, RedisError>
where
    K: ToRedisArgs + Send + Sync,
{
    let fields: Vec<String> = con.hkeys(&key).await?;

    if fields.is_empty() {
        return Ok(fields.len());
    }

    let deleted: usize = con.hdel(key, fields).await?;

    return Ok(deleted);
}

pub async fn insert_hmap<K, T>(
    con: &mut MultiplexedConnection,
    key: K,
    value: T,
) -> Result<usize, RedisError>
where
    K: ToRedisArgs + Send + Sync,
    T: ToMap,
{
    let map = T::to_stringmap(value).into_tuple_vec();

    let deleted: usize = con.hset_multiple(key, map.as_slice()).await?;

    return Ok(deleted);
}
