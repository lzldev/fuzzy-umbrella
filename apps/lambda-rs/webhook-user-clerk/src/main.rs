use artspace_core::env::EnvContainer;
use lambda_http::{
    http::StatusCode,
    run, service_fn,
    tracing::{self, info},
    Body, Error, Request, Response,
};
use serde_json::json;
use sqlx::{Connection, PgConnection};
use webhook_user_clerk::{
    clerk::{update_user_metadata, WebhookMessage},
    env::WebhookClerkEnvVars,
    webhooks::verify_webhook,
    PartialUser, WebhookClerkContext,
};
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let context = WebhookClerkContext::new().await;
    let context_ref = &context;

    run(service_fn(move |event| async move {
        let response = function_handler(event, context_ref).await?;

        Ok::<Response<Body>, Error>(response)
    }))
    .await
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(
    req: Request,
    context: &WebhookClerkContext,
) -> Result<Response<Body>, Error> {
    let secret = (&context.env).get_env_var(WebhookClerkEnvVars::WebhookSecret);

    let verify = verify_webhook(secret, &req);

    if let Err(_err) = verify {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Invalid Webhook signature".into())
            .unwrap());
    }

    let message: WebhookMessage =
        match serde_json::from_slice::<WebhookMessage>(req.body().as_ref()) {
            Ok(message) => message,
            Err(err) => {
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(format!("Invalid Webhook Message \n {err:#?}").into())
                    .unwrap())
            }
        };

    match message._type.as_str() {
        "user.created" => create_user(message, context).await,
        "user.updated" => update_user(message, context).await,
        _ => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Webhook Event not supported".into())
            .unwrap()),
    }
}

async fn create_user(
    event: WebhookMessage,
    context: &WebhookClerkContext,
) -> Result<Response<Body>, Error> {
    info!("USER.CREATED {:?}", &event);

    let email = event
        .data
        .email_addresses
        .first()
        .expect("User has no email address")
        .clone()
        .email_address;

    let new_user = sqlx::query_as::<_,PartialUser>("INSERT INTO users (clerk_id,username,email, image_url, clerk_updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING id,clerk_id")
        .bind(event.data.id)
        .bind(event.data.username.expect("Clerk user without Username"))
        .bind(email)
        .bind(event.data.image_url)
        .bind(chrono::DateTime::from_timestamp(event.data.updated_at as i64, 0))
        .fetch_one(context.database.lock().await.as_mut())
        .await
        .expect("To insert user");

    update_user_metadata(new_user, &context)
        .await
        .expect("to update user metadata");

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

async fn update_user(
    event: WebhookMessage,
    context: &WebhookClerkContext,
) -> Result<Response<Body>, Error> {
    info!("[USER.UPDATED] {:?}", &event);

    let email = event
        .data
        .email_addresses
        .first()
        .expect("User has no email address")
        .clone();

    let mut db = PgConnection::connect(
        context
            .env
            .get_env_var(WebhookClerkEnvVars::PostgresPoolURL)
            .as_str(),
    )
    .await
    .expect("db did not connect");

    let update_user = sqlx::query(r#"UPDATE users SET "username" = $1, "email" = $2, "image_url" = $3, "clerk_updated_at" = $4 WHERE "clerk_id" = $5"#)
        .bind(event.data.username)
        .bind(email.email_address)
        .bind(event.data.image_url)
        .bind(chrono::DateTime::from_timestamp(event.data.updated_at as i64, 0).expect("Couldn't parse event timestamp into DateTime"))
        .bind(event.data.id)
        .execute(&mut db)
        .await
        .expect("to update user").rows_affected();

    info!("Rows Updated: {}", update_user);

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
