use libsql::{self, Builder};
use mediathing_rust::WebhookMessage;
use std::env;

use serde_json::json;
use svix::webhooks::Webhook;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let wh_skip_verify = env::var("SKIP_VERIFY").is_ok();
    let wh_secret = env::var("WEBHOOK_SECRET").expect("Webhook secret not set");
    let turso_url = env::var("TURSO_CONNECTION_URL").expect("TURSO_CONNECTION_URL must be set");
    let turso_token = env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");

    let wh = Webhook::new(&wh_secret).unwrap();

    let verify = wh.verify(req.body(), req.headers());

    println!("[create.user]\n{:?}", req);
    if !wh_skip_verify {
        if let Err(_) = verify {
            println!("[create.user] Invalid signature");
            return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(
                json!({
                    "error": "Invalid signature",
                })
                .to_string()
                .into(),
            )?);
        }
    }
    println!("[create.user] Valid signature");
    let user = serde_json::from_slice::<WebhookMessage>(req.body().as_ref());

    if let Err(err) = user {
        return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(
            json!({
                "message":err.to_string(),
            })
            .to_string()
            .into(),
        )?);
    }

    let webhook_event = user.unwrap();
    println!("[create.user] {:?}", &webhook_event);

    let email = webhook_event
        .data
        .email_addresses
        .first()
        .expect("User has no email address")
        .clone();

    let db = Builder::new_remote(turso_url, turso_token).build().await?;
    let conn = db.connect().unwrap();

    conn.execute(
        "INSERT INTO users (clerk_id,clerk_username,email, image_url, clerk_updated_at) VALUES (?, ?, ?, ?, ?)",
        libsql::params![
            webhook_event.data.id,
            webhook_event.data.username,
            email.email_address,
            webhook_event.data.image_url,
            webhook_event.data.updated_at
        ],
    )
    .await?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": "User created",
            })
            .to_string()
            .into(),
        )?)
}
