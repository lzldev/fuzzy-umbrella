use artspace_core::env::{EnvContainer, EnvEnum, EnvMap};
use strum::EnumIter;

#[derive(EnumIter)]
pub enum LambdaEnv {
    WebhookSecret,
    ClerkSecret,
    TursoURL,
    TursoToken,
}

impl EnvEnum for LambdaEnv {
    fn var_name(var: &Self) -> &'static str {
        match var {
            LambdaEnv::WebhookSecret => "WEBHOOK_SECRET",
            LambdaEnv::ClerkSecret => "CLERK_SECRET_KEY",
            LambdaEnv::TursoURL => "TURSO_URL",
            LambdaEnv::TursoToken => "TURSO_TOKEN",
        }
    }
}

pub struct WebhookClerkEnv {
    map: EnvMap,
}

impl EnvContainer<LambdaEnv> for WebhookClerkEnv {
    fn get_map(&self) -> &EnvMap {
        &self.map
    }
    fn with_env_map(map: EnvMap) -> Self {
        Self { map }
    }
}
