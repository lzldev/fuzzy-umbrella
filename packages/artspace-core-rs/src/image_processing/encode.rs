use std::io::Write;

use image::{
    codecs::{jpeg::JpegEncoder, webp::WebPEncoder},
    ImageEncoder,
};

use super::process::ProcessingPlanType;

#[derive(Debug)]
pub struct EncodingResult {
    pub name: String,
    pub buf: Vec<u8>,
}

pub enum Encode<W> {
    Webp(WebPEncoder<W>),
    Jpeg(JpegEncoder<W>),
}

impl<W: Write> Encode<W> {
    pub fn from_processing_type(w: W, result: &ProcessingPlanType) -> Self {
        match result {
            ProcessingPlanType::Optimize => Self::Webp(WebPEncoder::new_lossless(w)),
            ProcessingPlanType::Thumbnail(_) => Self::Jpeg(JpegEncoder::new_with_quality(w, 63u8)),
        }
    }
}

impl<W: Write> ImageEncoder for Encode<W> {
    fn write_image(
        self,
        buf: &[u8],
        width: u32,
        height: u32,
        color_type: image::ExtendedColorType,
    ) -> image::ImageResult<()> {
        match self {
            Encode::Webp(e) => e.write_image(buf, width, height, color_type),
            Encode::Jpeg(e) => e.write_image(buf, width, height, color_type),
        }
    }
}
