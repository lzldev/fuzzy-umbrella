use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use webhook_user_clerk::WebhookClerkContext;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(
    event: Request,
    context: &WebhookClerkContext,
) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let context = WebhookClerkContext::new();
    let context_ref = &context;

    run(service_fn(move |event| async move {
        let response = function_handler(event, context_ref).await?;

        Ok::<Response<Body>, Error>(response)
    }))
    .await
}
