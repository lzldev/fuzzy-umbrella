use anyhow::anyhow;
use artspace_core::{
    image_processing::{
        process::{ProcessingPlan, ProcessingPlanType},
        process_image_vec,
    },
    redis::{keys::prepared_post_key, ArtsSpaceRedisCommands},
};

use artspace_shared::PreparedPost;
use aws_lambda_events::event::s3::S3Event;
use aws_sdk_s3::primitives::ByteStream;
use lambda_runtime::{
    run, service_fn,
    tracing::{self, debug, error, info},
    Error, LambdaEvent,
};

use lambda_s3_process::{
    env::{EnumMapEnv, LambdaEnv},
    utils::parse_object_key,
    SharedContext,
};

use tokio::task::JoinSet;
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let context = SharedContext::new().await;
    info!("Lambda Runtime STARTING");

    let context_ref = &context;

    run(service_fn(move |event| async move {
        let handler = function_handler(event, context_ref).await;

        if let Err(e) = &handler {
            error!("{e:?}");
        }

        handler
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
) -> Result<(), anyhow::Error> {
    // Extract some useful information from the request
    let payload = event.payload;

    info!("EVENT [{payload:?}]");

    let record = match payload.records.first() {
        Some(r) => r,
        None => {
            return Err(anyhow!("Event without a record"));
        }
    };

    if let Some(e) = &record.event_name {
        if !e.starts_with("ObjectCreated") {
            return Err(anyhow!("Event not supported"));
        }
    }

    let bucket_name = record.s3.bucket.name.clone().unwrap();
    let object_key = record.s3.object.key.clone().unwrap();

    let object = match context
        .s3_client
        .get_object()
        .bucket(bucket_name)
        .key(object_key.clone())
        .send()
        .await
    {
        Ok(object) => object,
        Err(e) => {
            error!("[GETObjectError] {:?}", e);
            return Err(anyhow!("Couldn't get Object"));
        }
    };

    debug!("Object[{object:?}]");

    let body: Vec<u8> = object
        .body
        .collect()
        .await
        .expect("Couldn't read Object body stream into memory")
        .into_bytes()
        .into();

    let object_key = parse_object_key(&object_key);
    debug!("object_key[{object_key}]");

    let plan = get_plan(object_key.as_str());
    debug!("Processing Plan [{plan:?}]");

    let output = process_image_vec(body, plan)
        .await
        .expect("Couldn't process images.");

    let output_bucket = context.env.get(LambdaEnv::OutputBucket).to_owned();

    let mut join_set = output
        .into_iter()
        .map(|thumb| {
            let output_bucket = output_bucket.clone();
            let obj = context.s3_client.put_object();

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

    let post = {
        let mut redis = context
            .redis_client
            .get_multiplexed_async_connection()
            .await?;

        let key = prepared_post_key(&object_key.as_str());
        debug!("{key:?}");

        if !redis.hmap_exists(&key).await? {
            return Err(anyhow!("PreparedPost not found in Redis."));
        };

        let post = redis.hmap_get::<_, PreparedPost>(&key).await?;

        if post.id.as_str() == "" {
            return Err(anyhow!("PreparedPost has no ID."));
        }

        tokio::spawn(async move {
            match redis.hmap_clear(&key).await {
                Ok(r) => info!("Redis Cleanup {r:?} keys deleted"),
                Err(e) => error!("Redis Clenup Error {e:?}"),
            };
        });

        post
    };

    Ok(())
}

//TODO: Make this a json configuration. imported from .... .somehwere.....
pub fn get_plan(object_key: &str) -> Vec<ProcessingPlan> {
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
