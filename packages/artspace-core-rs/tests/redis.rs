extern crate redis;

use std::collections::BTreeMap;

use anyhow::{anyhow, Context};
use artspace_core::{
    into_tuple_vec::IntoTupleVec,
    redis::{serde::ArtsSpaceRedisSerdeCommands, ArtsSpaceRedisCommands},
};
use artspace_shared::PreparedPost;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use structmap::ToMap;
use tokio::time::{Duration, Instant};

#[tokio::test]
async fn redis_connection() -> Result<(), anyhow::Error> {
    let client = redis::Client::open("redis://0.0.0.0:6379")?;

    let mut con = client.get_multiplexed_tokio_connection().await?;

    let post = PreparedPost {
        id: "8999".into(),
        content: "Hello World".into(),
        user_id: "clerk_id".into(),
    };

    let map = PreparedPost::to_stringmap(post);
    eprintln!("map:{map:?}");

    let map = map.into_tuple_vec();

    let added: bool = con.hset_multiple("jadas", map.as_slice()).await?;
    eprintln!("added:{added:?}");

    let exist: bool = con.hlen("jadas").await?;
    eprintln!("exist:{exist:?}");

    let keys: Vec<String> = con.hkeys("jadas").await?;

    eprintln!("keys:{keys:?}");

    if !keys.is_empty() {
        let del: usize = con.hdel("jadas", keys).await?;
        eprintln!("deleted:{del:?}");
    }

    let exist: bool = con.hlen("jadas").await?;
    eprintln!("exist:{exist:?}");
    Ok(())
}

#[tokio::test]
async fn serialize_post() -> Result<(), anyhow::Error> {
    let client = redis::Client::open("redis://0.0.0.0:6379")?;

    let mut con = client
        .get_multiplexed_async_connection_with_timeouts(
            Duration::from_millis(500),
            Duration::from_millis(500),
        )
        .await?;

    let post = PreparedPost {
        id: "12345".to_string(),
        content: "Hello World".to_string(),
        user_id: "54321".to_string(),
    };

    let serialized = serde_json::to_value(&post)?
        .as_object()
        .ok_or(anyhow!("not a object"))?
        .to_owned();

    let mapped = serialized
        .into_iter()
        .map(|(key, value)| (key, value.to_string()))
        .collect::<Vec<_>>();

    eprintln!("{mapped:?}");

    let key = "test-hash";

    con.hset_multiple(key, mapped.as_slice()).await?;

    let post: BTreeMap<String, String> = con.hgetall(key).await?;

    let post = post
        .into_iter()
        .map(|(key, value)| (key, serde_json::to_value(value).unwrap()))
        .collect::<BTreeMap<_, _>>();

    let serde_map = serde_json::Map::from_iter(post.into_iter());

    let value = serde_json::Value::from(serde_map);

    let post: PreparedPost = serde_json::from_value(value).unwrap();

    eprintln!("{post:?}");

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct TestStruct {
    id: usize,
    name: String,
    nullable: Option<usize>,
    truthy_nullable: Option<usize>,
    values: Vec<usize>,
    boolean: bool,
    number_like: String,
    boolean_like: String,
    empty_string: String,
}

#[tokio::test]
async fn serialize_test() -> Result<(), anyhow::Error> {
    let value = TestStruct {
        id: 100,
        name: "Hello World".to_owned(),
        nullable: None,
        truthy_nullable: Some(32),
        values: vec![1, 2, 3, 4, 5],
        boolean: false,
        number_like: "12345".to_owned(),
        boolean_like: "false".to_owned(),
        empty_string: "".to_owned(),
    };

    let clone = value.clone();

    let serialize_key = "serialize_key";
    let client = redis::Client::open("redis://0.0.0.0:6379")?;
    let mut con = client
        .get_multiplexed_async_connection_with_timeouts(
            Duration::from_millis(500),
            Duration::from_millis(500),
        )
        .await?;

    con.hmap_insert_serde(serialize_key, value).await?;
    let response: TestStruct = con.hmap_get_serde_value(serialize_key).await?;

    eprintln!("Clone:{clone:?}");
    eprintln!("Response:{response:?}");

    con.hmap_clear(serialize_key).await?;

    Ok(())
}

#[tokio::test]
async fn perf_test() -> Result<(), anyhow::Error> {
    let tests = 20000;
    let frommap_key = "bench-key";
    let serde_key = "serde-key";

    let client = redis::Client::open("redis://0.0.0.0:6379")?;
    let mut con = client
        .get_multiplexed_async_connection_with_timeouts(
            Duration::from_millis(500),
            Duration::from_millis(500),
        )
        .await?;

    let post = PreparedPost {
        id: "12345".to_string(),
        content: "Hello World".to_string(),
        user_id: "54321".to_string(),
    };

    let post2 = post.clone();
    con.hmap_insert(frommap_key, post)
        .await
        .context("insert hmap")?;

    let start = Instant::now();

    for _ in 0..tests {
        let _post: PreparedPost = con.hmap_get_frommap(frommap_key).await?;
        // eprintln!("{:?}", _post.id)
    }

    eprintln!("FromMap: {:?}", start.elapsed());

    con.hmap_insert_serde(serde_key, post2).await?;

    let start = Instant::now();

    for _ in 0..tests {
        let _post: PreparedPost =
            serde_json::from_value(con.hmap_get_serde_value(serde_key).await?)?;

        // eprintln!("{:?}", _post.id);
    }

    eprintln!("serde: {:?}", start.elapsed());

    con.hmap_clear(serde_key).await?;
    con.hmap_clear(frommap_key).await?;

    Ok(())
}
