use artspace_core::env::{EnvContainer, EnvEnum, EnvMap};
use strum::EnumIter;

#[derive(EnumIter)]
pub enum ProcessLambdaVars {
    OutputBucket,
    RedisURL,
    PostgresPoolURL,
}

impl EnvEnum for ProcessLambdaVars {
    fn var_name(var: &Self) -> &'static str {
        match var {
            Self::OutputBucket => "OUTPUT_BUCKET",
            Self::RedisURL => "REDIS_URL",
            Self::PostgresPoolURL => "DATABASE_URL",
        }
    }
}

pub struct ProcessLambdaEnv {
    map: EnvMap,
}

impl EnvContainer<ProcessLambdaVars> for ProcessLambdaEnv {
    fn get_map(&self) -> &EnvMap {
        &self.map
    }

    fn with_env_map(map: EnvMap) -> Self {
        Self { map }
    }
}
