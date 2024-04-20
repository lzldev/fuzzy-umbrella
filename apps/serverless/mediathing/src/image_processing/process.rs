use std::io::Write;

use image::DynamicImage;

use super::encode::{Encode, EncodingResult};

#[derive(Debug)]
pub struct ProcessingPlan {
    pub name: String,
    pub process: ProcessingPlanType,
}
impl ProcessingPlan {
    pub fn process_image(self, img: &DynamicImage) -> ProcessingResult {
        match self.process {
            ProcessingPlanType::Optimize => {
                let buf = img.clone();
                ProcessingResult::new_with_plan(self, buf)
            }
            ProcessingPlanType::Thumbnail(size) => {
                let buf = img.thumbnail(size.0, size.1);
                ProcessingResult::new_with_plan(self, buf)
            }
        }
    }
}

impl ProcessingResult {
    pub fn encode(self) -> EncodingResult {
        let mut sync_writer = std::io::BufWriter::new(vec![]);

        self.image
            .write_with_encoder(Encode::from_processing_type(
                sync_writer.by_ref(),
                &self.process,
            ))
            .unwrap();

        //todo:remove unwrap
        let buf = sync_writer.into_inner().unwrap();

        EncodingResult {
            name: self.name,
            buf,
        }
    }
}

#[derive(Debug)]
pub enum ProcessingPlanType {
    Optimize,
    Thumbnail((u32, u32)),
}

#[derive(Debug)]
pub struct ProcessingResult {
    pub name: String,
    pub process: ProcessingPlanType,
    pub image: DynamicImage,
}

impl ProcessingResult {
    fn new_with_plan(plan: ProcessingPlan, buf: DynamicImage) -> Self {
        Self {
            name: plan.name,
            process: plan.process,
            image: buf,
        }
    }
}
