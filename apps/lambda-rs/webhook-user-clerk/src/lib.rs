use artspace_core::env::EnvContainer;
use env::WebhookClerkEnv;

pub mod env;
pub mod webhooks;

pub struct WebhookClerkContext {
    pub env: WebhookClerkEnv,
}

impl WebhookClerkContext {
    pub fn new() -> Self {
        let env = WebhookClerkEnv::load_env();

        Self { env }
    }
}
