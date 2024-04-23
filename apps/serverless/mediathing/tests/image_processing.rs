use mediathing::image_processing::{
    process::{ProcessingPlan, ProcessingPlanType},
    process_image_vec,
};

use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt},
    task::JoinSet,
    time::Instant,
};

const TEST_INPUT_FILE_DIR: &str = "./tests/fixtures/object.png";
const TEST_OUTPUT_DIR: &str = "./tests/fixtures/";

#[tokio::test]
pub async fn test_image_processing_parallel() -> Result<(), anyhow::Error> {
    let mut open = fs::File::open(TEST_INPUT_FILE_DIR)
        .await
        .expect("Couldn't open test file.");

    let mut vec: Vec<u8> = Vec::new();
    open.read_to_end(&mut vec).await?;

    let name = "content";

    let plan = vec![
        ProcessingPlan {
            name: format!("{name}_optimized.webp"),
            process: ProcessingPlanType::Optimize,
        },
        ProcessingPlan {
            name: format!("{name}_thumb_small.jpeg"),
            process: ProcessingPlanType::Thumbnail((128, 128)),
        },
        ProcessingPlan {
            name: format!("{name}_thumb_medium.jpeg"),
            process: ProcessingPlanType::Thumbnail((256, 256)),
        },
        ProcessingPlan {
            name: format!("{name}_thumb_large.jpeg"),
            process: ProcessingPlanType::Thumbnail((512, 512)),
        },
    ];

    let t = Instant::now();
    let thumbs = process_image_vec(vec, plan).await?;
    eprintln!("Processing/Encoding : {:?}", t.elapsed());

    let t = Instant::now();
    let mut join_set = thumbs
        .into_iter()
        .map(|thumb| async move {
            let mut file = File::options()
                .write(true)
                .read(true)
                .create(true)
                .truncate(true)
                .open(format!("{TEST_OUTPUT_DIR}{}", &thumb.name))
                .await?;

            file.write_all(&thumb.buf).await?;

            Ok(thumb.name)
        })
        .collect::<JoinSet<Result<String, anyhow::Error>>>();

    while let Some(res) = join_set.join_next().await {
        res.unwrap().unwrap();
    }

    eprintln!("Writing : {:?}", t.elapsed());

    return Ok(());
}
