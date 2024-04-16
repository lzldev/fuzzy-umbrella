use std::{
    io::Cursor,
    ops::{Deref, Div},
    sync::{Arc, RwLock},
};

use image::{io::Reader as ImageReader, DynamicImage};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tokio::sync::oneshot;

pub fn load_image() {}

const THUBMNAILS: u32 = 4;
const TOTAL_SIZE: u32 = 300;

pub async fn process_image_vec(image_vec: Vec<u8>) -> Result<Vec<DynamicImage>, anyhow::Error> {
    //TODO: treat errors.
    let img = ImageReader::new(Cursor::new(image_vec))
        .with_guessed_format()?
        .decode()?;

    let img = Arc::new(RwLock::new(img));
    let (tx, rx) = oneshot::channel::<Vec<DynamicImage>>();

    rayon::spawn(move || {
        let ret = (0..THUBMNAILS)
            .into_par_iter()
            .map(|idx| {
                let img = &img.read().unwrap();
                if idx == 0 {
                    return img.deref().clone();
                }
                img.thumbnail(TOTAL_SIZE.div(idx + 1), TOTAL_SIZE.div(idx + 1))
            })
            .collect();

        tx.send(ret).expect("Couldn't  Send vec into channel");
        ()
    });

    Ok(rx.await.expect("Couldn't Receive From RAYON."))
}
