use artspace_core::env::EnvContainer;
use clerk_rs::{clerk::Clerk, ClerkConfiguration};
use env::{WebhookClerkEnv, WebhookClerkEnvVars};
use libsql::{Builder as TursoBuilder, Database as TursoDatabase};
use serde::{Deserialize, Serialize};

pub mod clerk;
pub mod env;
pub mod webhooks;

pub struct WebhookClerkContext {
    pub env: WebhookClerkEnv,
    pub database: TursoDatabase,
    pub clerk_client: Clerk,
}

impl WebhookClerkContext {
    pub async fn new() -> Self {
        let env = WebhookClerkEnv::load_env();

        let clerk_config = ClerkConfiguration::new(
            None,
            None,
            Some(env.get_env_var(WebhookClerkEnvVars::ClerkSecret)),
            None,
        );

        let clerk_client = Clerk::new(clerk_config);

        let database = TursoBuilder::new_remote(
            env.get_env_var(WebhookClerkEnvVars::TursoURL),
            env.get_env_var(WebhookClerkEnvVars::TursoToken),
        )
        .build()
        .await
        .expect("Couldn't open connection to turso database.");

        Self {
            env,
            database,
            clerk_client,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialUser {
    pub id: usize,
    pub clerk_id: String,
}
