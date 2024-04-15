use std::io::Read;

use aws_config::SdkConfig;
use aws_lambda_events::event::s3::S3Event;
use aws_sdk_s3::Client;
use fn_s3_process::LambdaEnv;
use lambda_runtime::{
    run, service_fn,
    tracing::{self, debug, error, info},
    Error, LambdaEvent,
};
use tokio::{
    fs,
    io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt},
};

struct SharedContext {
    config: SdkConfig,
    client: Client,
    env: LambdaEnv,
}

impl SharedContext {
    pub async fn new() -> Self {
        let env = LambdaEnv::new();
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_s3::Client::new(&config);

        Self {
            env,
            config,
            client,
        }
    }
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(
    event: LambdaEvent<S3Event>,
    context: &SharedContext,
) -> Result<(), Error> {
    // Extract some useful information from the request
    let payload = event.payload;

    let record = match payload.records.first() {
        Some(r) => r,
        None => {
            return Err("Event without a record".into());
        }
    };

    if Some(String::from("ObjectCreated:Post")) != record.event_name {
        return Err("Event not supported".into());
    }

    let bucket_name = record.s3.bucket.name.clone().unwrap();
    let object_key = record.s3.object.key.clone().unwrap();

    let object = match context
        .client
        .get_object()
        .bucket(bucket_name)
        .key(object_key)
        .send()
        .await
    {
        Err(e) => {
            error!("[GETObjectError] {:?}", e);
            return Err("Couldn't get Object".into());
        }
        Ok(object) => object,
    };

    info!("Object: {:?}", object);

    let mut reader = object.body.into_async_read();
    let mut out = fs::File::options()
        .write(true)
        .create(true)
        .create_new(true)
        .open("/tmp/object.png")
        .await
        .expect("To Open File");

    io::copy(&mut reader, &mut out)
        .await
        .expect("To Write file");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    let context = SharedContext::new().await;
    let context_ref = &context;
    info!("Lambda Runtime Starting");

    run(service_fn(move |event| {
        return function_handler(event, context_ref);
    }))
    .await
}
