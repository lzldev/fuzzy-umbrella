use std::collections::BTreeMap;

use redis::{AsyncCommands, RedisError, ToRedisArgs};

use structmap::{FromMap, ToMap};

use crate::into_tuple_vec::IntoTupleVec;
#[allow(async_fn_in_trait)]

pub trait ArtsSpaceRedisCommands
where
    Self: AsyncCommands,
{
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

    async fn hmap_insert<K, T>(&mut self, key: K, value: T) -> Result<(), RedisError>
    where
        K: ToRedisArgs + Send + Sync,
        T: ToMap + Send,
    {
        let map = T::to_stringmap(value).into_tuple_vec();

        self.hset_multiple(key, map.as_slice()).await?;

        return Ok(());
    }

    async fn hmap_exists<K>(&mut self, key: K) -> Result<bool, RedisError>
    where
        K: ToRedisArgs + Send + Sync,
    {
        let len: usize = self.hlen(key).await?;

        if len == 0 {
            return Ok(false);
        }

        return Ok(true);
    }

    async fn hmap_get_frommap<K, T>(&mut self, key: K) -> Result<T, RedisError>
    where
        K: ToRedisArgs + Send + Sync,
        T: FromMap,
    {
        let map: BTreeMap<String, String> = self.hgetall(key).await?;

        Ok(T::from_stringmap(map))
    }
}

impl<T> ArtsSpaceRedisCommands for T where T: AsyncCommands {}
pub mod serde {
    use std::{collections::BTreeMap, str::FromStr};

    use anyhow::{anyhow, Context};
    use redis::{AsyncCommands, ToRedisArgs};
    use serde::{de::DeserializeOwned, Serialize};

    #[allow(async_fn_in_trait)]
    pub trait ArtsSpaceRedisSerdeCommands
    where
        Self: AsyncCommands,
    {
        async fn hmap_insert_serde<K, T>(&mut self, key: K, value: T) -> Result<(), anyhow::Error>
        where
            K: ToRedisArgs + Send + Sync,
            T: Serialize,
        {
            let map: Vec<(String, String)> = serde_json::to_value(value)?
                .as_object()
                .ok_or(anyhow!("Trying to insert non Object value as Redis HMAP"))?
                .to_owned()
                .into_iter()
                .map(|(k, v)| {
                    // (k, v.to_string())
                    (
                        k,
                        match v {
                            // serde_json::Value::Null => v.to_string(),
                            // serde_json::Value::Bool(v) => {
                            //     serde_json::to_string(&v).expect("To serde bool into string.")
                            // }
                            // serde_json::Value::Number(v) => v.to_string(),
                            serde_json::Value::String(v) => v,
                            v => v.to_string(),
                        },
                    )
                })
                .collect();

            self.hset_multiple(key, map.as_slice()).await?;

            Ok(())
        }

        async fn hmap_get_serde_value<K, T>(&mut self, key: K) -> Result<T, anyhow::Error>
        where
            K: ToRedisArgs + Send + Sync,
            T: DeserializeOwned,
        {
            let map: BTreeMap<String, String> = self.hgetall(key).await?;

            let iter = map.into_iter().map(|(key, value)| {
                (
                    key,
                    serde_json::Value::from_str(&value)
                        .unwrap_or_else(|_| serde_json::Value::String(value)),
                )
            });

            let map = serde_json::Map::from_iter(iter);

            Ok(serde_json::from_value(serde_json::Value::from(map))
                .context("Redis value couldn't be deserialized.")?)
        }
    }

    impl<T> ArtsSpaceRedisSerdeCommands for T where T: AsyncCommands {}
}
