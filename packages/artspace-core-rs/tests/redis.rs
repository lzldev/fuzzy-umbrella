use artspace_core::into_tuple_vec::IntoTupleVec;
use artspace_shared::PreparedPost;
use redis::AsyncCommands;
use structmap::ToMap;

extern crate redis;

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
