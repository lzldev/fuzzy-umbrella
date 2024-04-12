use libsql::{self, Builder};
use mediathing_rust::{verify_webhook, WebhookMessage};
use std::env;

use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let turso_url = env::var("TURSO_CONNECTION_URL").expect("TURSO_CONNECTION_URL must be set");
    let turso_token = env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");

    if let Err(_) = verify_webhook(&req) {
        println!("[user.updated] Invalid signature");
        return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(
            json!({
                "error": "Invalid signature",
            })
            .to_string()
            .into(),
        )?);
    }

    println!("[user.updated] Valid signature");
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
    println!("[user.updated] {:?}", &webhook_event);

    let email = webhook_event
        .data
        .email_addresses
        .first()
        .expect("User has no email address")
        .clone();

    let db = Builder::new_remote(turso_url, turso_token).build().await?;
    let conn = db.connect().unwrap();

    conn.execute(
        r#"UPDATE users SET "username" = ?, "email" = ?, "image_url" = ?, "clerk_updated_at" = ? WHERE "clerk_id" = ?"#,
        libsql::params![
            webhook_event.data.username,
            email.email_address,
            webhook_event.data.image_url,
            webhook_event.data.updated_at,
            webhook_event.data.id,
        ],
    )
    .await?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": "user created",
            })
            .to_string()
            .into(),
        )?)
}
