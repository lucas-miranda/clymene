use std::path::Path;

use crate::{
    processors::{
        image::format_handlers::{
            Error,
            FormatProcessor
        },
        ConfigStatus
    },
    settings::Config
};

#[derive(Default)]
pub struct RawFileProcessor {
}

impl FormatProcessor for RawFileProcessor {
    fn setup(&self, config: &mut Config) -> Result<ConfigStatus, Error> {
        Ok(ConfigStatus::NotModified)
    }

    fn process(&self, source_file_path: &Path, output_dir_path: &Path, config: &Config) -> Result<Graphic, Error> {
        panic!()
    }
}
