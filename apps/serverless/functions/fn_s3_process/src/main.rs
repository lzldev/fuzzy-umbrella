use aws_config::SdkConfig;
use aws_lambda_events::event::s3::S3Event;
use aws_sdk_s3::{primitives::ByteStream, Client};
use fn_s3_process::{EnumMapEnv, EnvTwo, LambdaEnv};
use lambda_runtime::{
    run, service_fn,
    tracing::{self, error, info},
    Error, LambdaEvent,
};
use mediathing::image_processing::{
    process::{ProcessingPlan, ProcessingPlanType},
    process_image_vec,
};
use tokio::task::JoinSet;

struct SharedContext<'a> {
    env: EnvTwo<'a>,
    config: SdkConfig,
    client: Client,
}

impl<'ctx> SharedContext<'ctx> {
    pub async fn new() -> Self {
        let env = EnvTwo::load_env();
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_s3::Client::new(&config);

        Self {
            env,
            config,
            client,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    let context = SharedContext::new().await;
    info!("Lambda Runtime Starting");

    let context_ref = &context;

    run(service_fn(move |event| async move {
        function_handler(event, context_ref).await
    }))
    .await
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler<'a>(
    event: LambdaEvent<S3Event>,
    context: &'a SharedContext<'a>,
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
        .key(object_key.clone())
        .send()
        .await
    {
        Err(e) => {
            error!("[GETObjectError] {:?}", e);
            return Err("Couldn't get Object".into());
        }
        Ok(object) => object,
    };

    info!("Object: \n{:?}", object);

    let body: Vec<u8> = object
        .body
        .collect()
        .await
        .expect("Couldn't read Object body stream into memory")
        .into_bytes()
        .into();

    let object_key = object_key
        .split(".")
        .nth(0)
        .expect("Couldn't Get Object name")
        .to_owned();

    let plan = get_plan(object_key);

    let output = process_image_vec(body, plan)
        .await
        .expect("Couldn't process images.");

    let output_bucket = context.env.get(LambdaEnv::OutputBucket).to_owned();

    let mut join_set = output
        .into_iter()
        .map(|thumb| {
            let output_bucket = output_bucket.clone();
            let obj = context.client.put_object();

            async move {
                let output_bucket = output_bucket.clone();
                let obj_key = thumb.name.clone();
                let body_stream = ByteStream::from(thumb.buf);

                obj.bucket(output_bucket)
                    .key(obj_key)
                    .body(body_stream)
                    .send()
                    .await
                    .map_err(|e| anyhow::Error::new(e))?;

                Ok::<String, anyhow::Error>(thumb.name)
            }
        })
        .collect::<JoinSet<_>>();

    while let Some(join) = join_set.join_next().await {
        match join {
            Ok(Ok(r)) => {
                info!("Uploaded {r}");
            }
            Ok(Err(e)) => {
                error!("Error Putting objects into Content Bucket");
                error!("{e:?}");

                join_set.shutdown().await;
                break;
            }
            Err(e) => {
                error!("Join Error while uploading objects");
                error!("{e:?}");

                join_set.shutdown().await;
                break;
            }
        }
    }

    Ok(())
}

//TODO: Make this a json configuration. imported from .... .somehwere.....
pub fn get_plan(object_key: String) -> Vec<ProcessingPlan> {
    vec![
        ProcessingPlan {
            name: format!("{object_key}_optimized.webp"),
            process: ProcessingPlanType::Optimize,
        },
        ProcessingPlan {
            name: format!("{object_key}_thumb_small.jpeg"),
            process: ProcessingPlanType::Thumbnail((128, 128)),
        },
        ProcessingPlan {
            name: format!("{object_key}_thumb_medium.jpeg"),
            process: ProcessingPlanType::Thumbnail((256, 256)),
        },
        ProcessingPlan {
            name: format!("{object_key}_thumb_large.jpeg"),
            process: ProcessingPlanType::Thumbnail((512, 512)),
        },
    ]
}