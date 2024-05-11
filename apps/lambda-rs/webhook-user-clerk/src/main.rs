use artspace_core::env::EnvContainer;
use lambda_http::{
    http::StatusCode,
    run, service_fn,
    tracing::{self, info},
    Body, Error, Request, Response,
};
use serde_json::json;
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
        .clone();

    let db = context.database.connect().unwrap();
    let clerk_id = event.data.id;

    let mut rows = db.query(
        "INSERT INTO users (clerk_id,username,email, image_url, clerk_updated_at) VALUES (?, ?, ?, ?, ?) RETURNING id,clerk_id",
        libsql::params![
            clerk_id,
            event.data.username,
            email.email_address,
            event.data.image_url,
            event.data.updated_at
        ],
    )
    .await?;

    let new_user = libsql::de::from_row::<PartialUser>(
        &rows
            .next()
            .await
            .expect("Insert query didn't new value.")
            .unwrap(),
    )
    .expect("Couldn't serialize partial user");

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

    let db = context.database.connect().unwrap();

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
