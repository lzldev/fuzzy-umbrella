use std::env;

pub struct LambdaEnv {
    pub output_bucket: String,
}

impl LambdaEnv {
    pub fn new() -> Self {
        let output_bucket =
            env::var("OUTPUT_BUCKET").expect("OUTPUT_BUCKET env var is not defined.");

        Self { output_bucket }
    }
}

impl Default for LambdaEnv {
    fn default() -> Self {
        Self::new()
    }
}
