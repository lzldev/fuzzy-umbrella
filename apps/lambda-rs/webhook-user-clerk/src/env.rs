use artspace_core::env::{EnvContainer, EnvEnum, EnvMap};
use strum::EnumIter;

#[derive(EnumIter)]
pub enum WebhookClerkEnvVars {
    WebhookSecret,
    ClerkSecret,
    TursoURL,
    TursoToken,
}

impl EnvEnum for WebhookClerkEnvVars {
    fn var_name(var: &Self) -> &'static str {
        match var {
            WebhookClerkEnvVars::WebhookSecret => "WEBHOOK_SECRET",
            WebhookClerkEnvVars::ClerkSecret => "CLERK_SECRET_KEY",
            WebhookClerkEnvVars::TursoURL => "TURSO_URL",
            WebhookClerkEnvVars::TursoToken => "TURSO_TOKEN",
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
