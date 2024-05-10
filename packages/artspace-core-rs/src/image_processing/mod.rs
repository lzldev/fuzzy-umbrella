pub mod encode;
pub mod process;

use std::{
    io::Cursor,
    sync::{Arc, RwLock},
};

use image::io::Reader as ImageReader;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tokio::sync::oneshot;

use crate::image_processing::process::ProcessingResult;

use self::{encode::EncodingResult, process::ProcessingPlan};

pub async fn process_image_vec(
    image_vec: Vec<u8>,
    plan: Vec<ProcessingPlan>,
) -> Result<Vec<EncodingResult>, anyhow::Error> {
    let img = ImageReader::new(Cursor::new(image_vec))
        .with_guessed_format()?
        .decode()?;

    let img = Arc::new(RwLock::new(img));
    let (tx, rx) = oneshot::channel::<Vec<EncodingResult>>();

    rayon::spawn(move || {
        let ret: Vec<EncodingResult> = plan
            .into_par_iter()
            .map(|plan| {
                let img = &img.read().expect("Image lock is poisoned");

                let processed = plan.process_image(img);

                let processed = ProcessingResult {
                    //TODO:Refactor this lmao.
                    image: processed.image.into_rgb8().into(),
                    ..processed
                };

                let encoded = processed.encode();

                encoded
            })
            .collect();

        tx.send(ret).expect("Couldn't  Send vec into channel");
    });

    Ok(rx.await?)
}
