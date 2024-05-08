use env::EnvTwo;
use lambda_runtime::tracing::info;
use libsql::{Builder as TursoBuilder, Database as TursoDatabase};
use tokio::time::{Duration, Instant};

use crate::env::{EnumMapEnv, LambdaEnv};
pub mod env;
pub mod utils;

use aws_sdk_s3::Client as S3Client;
use redis::Client as RedisClient;

pub struct SharedContext<'a> {
    pub env: EnvTwo<'a>,
    pub s3_client: S3Client,
    pub redis_client: RedisClient,
    pub database: TursoDatabase,
}

impl<'ctx> SharedContext<'ctx> {
    pub async fn new() -> Self {
        let aws_config = aws_config::load_from_env().await;
        let env = EnvTwo::load_env();

        let redis_url = env.get(LambdaEnv::RedisURL).as_str();

        let redis_client =
            RedisClient::open(redis_url).expect("Couldn't open connection to redis.");

        info!("Connected to redis on : {redis_url}");
        let start = Instant::now();
        let mut con = redis_client
            .get_multiplexed_async_connection_with_timeouts(
                Duration::from_secs_f64(0.5),
                Duration::from_secs_f64(0.5),
            )
            .await
            .expect("To Open REDIS test Connection");

        con.send_packed_command(&redis::cmd("PING"))
            .await
            .expect("Pinging Redis");

        info!("Redis pinged in {:?}", start.elapsed());

        let database = TursoBuilder::new_remote(
            env.get(LambdaEnv::TursoURL).to_owned(),
            env.get(LambdaEnv::TursoToken).to_owned(),
        )
        .build()
        .await
        .expect("Couldn't open connection to turso database.");

        Self {
            env,
            database,
            redis_client,
            s3_client: S3Client::new(&aws_config),
        }
    }
}
