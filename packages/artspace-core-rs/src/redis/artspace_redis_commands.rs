use redis::{aio::MultiplexedConnection, AsyncCommands, RedisError, ToRedisArgs};

use structmap::ToMap;

use crate::into_tuple_vec::IntoTupleVec;

pub trait ArtsSpaceRedisCommands
where
    Self: AsyncCommands,
{
    async fn clear_hmap<K>(&mut self, key: K) -> Result<usize, RedisError>
    where
        K: ToRedisArgs + Send + Sync;

    async fn insert_hmap<K, T>(&mut self, key: K, value: T) -> Result<usize, RedisError>
    where
        K: ToRedisArgs + Send + Sync,
        T: ToMap;

    async fn key_exists<K>(&mut self, key: K) -> Result<bool, RedisError>
    where
        K: ToRedisArgs + Send + Sync;
}

impl ArtsSpaceRedisCommands for MultiplexedConnection {
    async fn clear_hmap<K>(&mut self, key: K) -> Result<usize, RedisError>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let fields: Vec<String> = self.hkeys(&key).await?;

        if fields.is_empty() {
            return Ok(fields.len());
        }

        let deleted: usize = self.hdel(key, fields).await?;

        return Ok(deleted);
    }

    async fn insert_hmap<K, T>(&mut self, key: K, value: T) -> Result<usize, RedisError>
    where
        K: ToRedisArgs + Send + Sync,
        T: ToMap,
    {
        let map = T::to_stringmap(value).into_tuple_vec();

        let deleted: usize = self.hset_multiple(key, map.as_slice()).await?;

        return Ok(deleted);
    }

    async fn key_exists<K>(&mut self, key: K) -> Result<bool, RedisError>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let len: usize = self.hlen(key).await?;

        if len == 0 {
            return Ok(false);
        }

        return Ok(false);
    }
}
