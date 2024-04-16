use mediathing::image_processing::process_image_vec;
use tokio::{fs, io::AsyncReadExt};

const FILE: &str =
    "/home/rei/code/personal/mediathing/apps/serverless/mediathing/tests/fixtures/object.png";

#[tokio::test]
pub async fn test_image_processing() -> Result<(), anyhow::Error> {
    let mut open = fs::File::open(FILE)
        .await
        .expect("Couldn't open test file.");
    let mut vec: Vec<u8> = Vec::new();
    open.read_to_end(&mut vec).await?;

    let thumbs = process_image_vec(vec).await?;

    thumbs.iter().enumerate().for_each(|(idx, thumb)| {
        thumb
            .save_with_format(
                format!("./tests/fixtures/thumb{}.webp", idx).as_str(),
                image::ImageFormat::WebP,
            )
            .unwrap();
    });

    return Ok(());
}
