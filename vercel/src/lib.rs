mod schema;
use std::env;

pub use schema::*;
use svix::webhooks::{Webhook, WebhookError};
use vercel_runtime::Request;

pub fn verify_webhook(req: &Request) -> Result<(), WebhookError> {
    let wh_skip_verify = env::var("SKIP_VERIFY").is_ok();

    if wh_skip_verify {
        eprintln!("[Webhook] Skipping signature verification");
        return Ok(());
    }

    let wh_secret = env::var("WEBHOOK_SECRET").expect("WEBHOOK_SECRET not set");
    let wh = Webhook::new(&wh_secret).expect("WEBHOOK_SECRET not valid");

    let verify = wh.verify(req.body(), req.headers());

    if verify.is_err() {
        return verify;
    }

    return Ok(());
}
