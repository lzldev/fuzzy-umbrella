use artspace_core::env::EnvContainer;
use env::{ProcessLambdaEnv, ProcessLambdaVars};
use lambda_runtime::tracing::info;
use sqlx::{Connection, PgConnection};
use tokio::{
    sync::Mutex,
    time::{Duration, Instant},
};

pub mod env;
pub mod utils;

use aws_sdk_s3::Client as S3Client;
use redis::Client as RedisClient;

pub struct SharedContext {
    pub env: ProcessLambdaEnv,
    pub s3_client: S3Client,
    pub redis_client: RedisClient,
    pub database: Mutex<PgConnection>,
}

impl SharedContext {
    pub async fn new() -> Self {
        let aws_config = aws_config::load_from_env().await;
        let env = ProcessLambdaEnv::load_env();

        let redis_url = env.get_env_var(ProcessLambdaVars::RedisURL);

        let redis_client =
            RedisClient::open(redis_url.as_str()).expect("Couldn't open connection to redis.");

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

        let mut database =
            PgConnection::connect(env.get_env_var(ProcessLambdaVars::PostgresPoolURL).as_str())
                .await
                .expect("To Create Database Connection");

        database.ping().await.expect("To Ping database");

        Self {
            env,
            redis_client,
            database: Mutex::new(database),
            s3_client: S3Client::new(&aws_config),
        }
    }
}
