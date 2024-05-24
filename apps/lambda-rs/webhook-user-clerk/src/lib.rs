use artspace_core::env::EnvContainer;
use clerk_rs::{clerk::Clerk, ClerkConfiguration};
use env::{WebhookClerkEnv, WebhookClerkEnvVars};
use serde::{Deserialize, Serialize};
use sqlx::{Connection, PgConnection};
use tokio::sync::Mutex;

pub mod clerk;
pub mod env;
pub mod webhooks;

pub struct WebhookClerkContext {
    pub env: WebhookClerkEnv,
    pub database: Mutex<PgConnection>,
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

        let mut database = PgConnection::connect(
            env.get_env_var(WebhookClerkEnvVars::PostgresPoolURL)
                .as_str(),
        )
        .await
        .expect("Couldn't connect to database");

        database.ping().await.expect("Couldn't ping database");

        let clerk_client = Clerk::new(clerk_config);

        Self {
            env,
            clerk_client,
            database: Mutex::new(database),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PartialUser {
    pub id: sqlx::types::Uuid,
    pub clerk_id: String,
}
