use artspace_core::env::EnvContainer;
use lambda_http::{
    http::StatusCode,
    run, service_fn,
    tracing::{self, info},
    Body, Error, Request, Response,
};
use serde_json::json;
use webhook_user_clerk::{
    clerk::WebhookMessage, env::WebhookClerkEnvVars, webhooks::verify_webhook, WebhookClerkContext,
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
    // Extract some useful information from the request

    let secret = (&context.env).get_env_var(WebhookClerkEnvVars::ClerkSecret);

    let verify = verify_webhook(secret, &req);

    if let Err(_err) = verify {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Invalid Webhook signature".into())
            .unwrap());
    }

    let message: WebhookMessage = {
        match serde_json::from_slice::<WebhookMessage>(req.body().as_ref()) {
            Ok(message) => message,
            Err(err) => {
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(format!("Invalid Webhook Message \n {err:#?}").into())
                    .unwrap())
            }
        }
    };

    match message._type.as_str() {
        "user.created" => create_user(message, context).await,
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
    info!("[user.created] {:?}", &event);

    let email = event
        .data
        .email_addresses
        .first()
        .expect("User has no email address")
        .clone();

    let db = context.database.connect().unwrap();

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
