use artspace_core::env::EnvContainer;
use env::{WebhookClerkEnv, WebhookClerkEnvVars};
use libsql::{Builder as TursoBuilder, Database as TursoDatabase};

pub mod clerk;
pub mod env;
pub mod webhooks;

pub struct WebhookClerkContext {
    pub env: WebhookClerkEnv,
    pub database: TursoDatabase,
}

impl WebhookClerkContext {
    pub async fn new() -> Self {
        let env = WebhookClerkEnv::load_env();

        let database = TursoBuilder::new_remote(
            env.get_env_var(WebhookClerkEnvVars::TursoURL),
            env.get_env_var(WebhookClerkEnvVars::TursoURL),
        )
        .build()
        .await
        .expect("Couldn't open connection to turso database.");

        Self { env, database }
    }
}
