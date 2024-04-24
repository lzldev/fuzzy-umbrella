use std::collections::BTreeMap;

use redis::{aio::MultiplexedConnection, AsyncCommands, RedisError, ToRedisArgs};

use structmap::{FromMap, ToMap};

use crate::into_tuple_vec::IntoTupleVec;

#[allow(async_fn_in_trait)]
pub trait ArtsSpaceRedisCommands
where
    Self: AsyncCommands,
{
    async fn hmap_clear<K>(&mut self, key: K) -> Result<usize, RedisError>
    where
        K: ToRedisArgs + Send + Sync;

    async fn hmap_insert<K, T>(&mut self, key: K, value: T) -> Result<usize, RedisError>
    where
        K: ToRedisArgs + Send + Sync,
        T: ToMap + Send;

    async fn hmap_exists<K>(&mut self, key: K) -> Result<bool, RedisError>
    where
        K: ToRedisArgs + Send + Sync;

    async fn hmap_get<K, T>(&mut self, key: K) -> Result<T, RedisError>
    where
        K: ToRedisArgs + Send + Sync,
        T: FromMap;
}

impl ArtsSpaceRedisCommands for MultiplexedConnection {
    async fn hmap_clear<K>(&mut self, key: K) -> Result<usize, RedisError>
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

    async fn hmap_insert<K, T>(&mut self, key: K, value: T) -> Result<usize, RedisError>
    where
        K: ToRedisArgs + Send + Sync,
        T: ToMap + Send,
    {
        let map = T::to_stringmap(value).into_tuple_vec();

        let inserted: usize = self.hset_multiple(key, map.as_slice()).await?;

        return Ok(inserted);
    }

    async fn hmap_exists<K>(&mut self, key: K) -> Result<bool, RedisError>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let len: usize = self.hlen(key).await?;

        if len == 0 {
            return Ok(false);
        }

        return Ok(false);
    }

    async fn hmap_get<K, T>(&mut self, key: K) -> Result<T, RedisError>
    where
        K: ToRedisArgs + Send + Sync,
        T: FromMap,
    {
        let map: BTreeMap<String, String> = self.hgetall(key).await?;

        Ok(T::from_stringmap(map))
    }
}
