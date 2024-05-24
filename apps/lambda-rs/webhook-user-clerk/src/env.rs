use artspace_core::env::{EnvContainer, EnvEnum, EnvMap};
use strum::EnumIter;

#[derive(EnumIter)]
pub enum WebhookClerkEnvVars {
    WebhookSecret,
    ClerkSecret,
    PostgresPoolURL,
}

impl EnvEnum for WebhookClerkEnvVars {
    fn var_name(var: &Self) -> &'static str {
        match var {
            WebhookClerkEnvVars::WebhookSecret => "WEBHOOK_SECRET",
            WebhookClerkEnvVars::PostgresPoolURL => "DATABASE_URL",
            WebhookClerkEnvVars::ClerkSecret => "CLERK_SECRET_KEY",
        }
    }
}

pub struct WebhookClerkEnv {
    map: EnvMap,
}

impl EnvContainer<WebhookClerkEnvVars> for WebhookClerkEnv {
    fn get_map(&self) -> &EnvMap {
        &self.map
    }
    fn with_env_map(map: EnvMap) -> Self {
        Self { map }
    }
}
