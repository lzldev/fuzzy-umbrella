use mediathing_rust::{db::DB, env::validate_env, verify_webhook, WebhookMessage};

use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    validate_env();
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    if let Err(_) = verify_webhook(&req) {
        println!("[clerk] Invalid signature");
        return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(
            json!({
                "error": "Invalid signature",
            })
            .to_string()
            .into(),
        )?);
    }
    
    println!("[clerk] Valid signature");
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

    let event = user.unwrap();

    match event._type.as_str() {
        "user.created" => create_user(event, req).await,
        "user.updated" => update_user(event, req).await,
        unknown_type => {
            println!("[clerk] Unknown event type [{}]", unknown_type);
            println!("{:?}", event);

            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .body(
                    json!({
                        "message": "Event not supported",
                    })
                    .to_string()
                    .into(),
                )?);
        }
    }
}

pub async fn create_user(event: WebhookMessage, _: Request) -> Result<Response<Body>, Error> {
    println!("[user.created] {:?}", &event);

    let email = event
        .data
        .email_addresses
        .first()
        .expect("User has no email address")
        .clone();

    let db = DB::connect().await;

    db.execute(
        "INSERT INTO users (clerk_id,username,email, image_url, clerk_updated_at) VALUES (?, ?, ?, ?, ?)",
        libsql::params![
            event.data.id,
            event.data.username,
            email.email_address,
            event.data.image_url,
            event.data.updated_at
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

pub async fn update_user(event: WebhookMessage, _: Request) -> Result<Response<Body>, Error> {
    println!("[user.updated] {:?}", &event);

    let email = event
        .data
        .email_addresses
        .first()
        .expect("User has no email address")
        .clone();

    let db = DB::connect().await;

    db.execute(
        r#"UPDATE users SET "username" = ?, "email" = ?, "image_url" = ?, "clerk_updated_at" = ? WHERE "clerk_id" = ?"#,
        libsql::params![
            event.data.username,
            email.email_address,
            event.data.image_url,
            event.data.updated_at,
            event.data.id,
        ],
    )
    .await?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": "user updated",
            })
            .to_string()
            .into(),
        )?)
}
