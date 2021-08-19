use std::path::Path;

use crate::{
    graphics::Graphic,
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
    fn setup(&self, _config: &mut Config) -> Result<ConfigStatus, Error> {
        Ok(ConfigStatus::NotModified)
    }

    fn process(&self, _source_file_path: &Path, _output_dir_path: &Path, _config: &Config) -> Result<Graphic, Error> {
        panic!()
    }
}
