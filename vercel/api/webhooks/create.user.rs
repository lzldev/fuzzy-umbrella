use libsql::{self, Builder};
use std::env;

use serde_json::json;
use svix::webhooks::Webhook;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let wh_secret = env::var("WEBHOOK_SECRET").expect("Webhook secret not set");
    let turso_url = env::var("TURSO_CONNECTION_URL").expect("TURSO_CONNECTION_URL must be set");
    let turso_token = env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");

    let wh = Webhook::new(&wh_secret).unwrap();

    let verify = wh.verify(req.body(), req.headers());

    println!("[create.user]\n{:?}", req);
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

    let db = Builder::new_remote(turso_url, turso_token).build().await?;
    let conn = db.connect().unwrap();

    println!("[create.user] Valid signature");

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "data": "pong",
            })
            .to_string()
            .into(),
        )?)
}
