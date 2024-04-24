use env::EnvTwo;
use lambda_runtime::tracing::info;
use libsql::{Builder as TursoBuilder, Database as TursoDatabase};

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

        let redis_host = env.get(LambdaEnv::RedisHost).as_str();

        let redis_client =
            RedisClient::open(redis_host).expect("Couldn't open connection to redis.");

        let database = TursoBuilder::new_remote(
            env.get(LambdaEnv::TursoURL).to_owned(),
            env.get(LambdaEnv::TursoToken).to_owned(),
        )
        .build()
        .await
        .expect("Couldn't open connection to turso database.");

        info!("Connected to redis on : {redis_host}");

        Self {
            env,
            database,
            redis_client,
            s3_client: S3Client::new(&aws_config),
        }
    }
}
