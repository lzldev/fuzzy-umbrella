use artspace_core::env::{EnvContainer, EnvEnum, EnvMap};
use strum::EnumIter;

#[derive(Debug, EnumIter)]
pub enum WSBackendEnvVars {
    CORSOrigins,
    ClerkJWKSUrl,
}

impl EnvEnum for WSBackendEnvVars {
    fn var_name(var: &Self) -> &'static str {
        match var {
            WSBackendEnvVars::CORSOrigins => "WS_CORS_ORIGINS",
            WSBackendEnvVars::ClerkJWKSUrl => "CLERK_JWKS_URL",
        }
    }
}

pub struct WSBackendEnv {
    map: EnvMap,
}

impl EnvContainer<WSBackendEnvVars> for WSBackendEnv {
    fn get_map(&self) -> &EnvMap {
        &self.map
    }
    fn with_env_map(map: EnvMap) -> Self {
        Self { map }
    }
}
