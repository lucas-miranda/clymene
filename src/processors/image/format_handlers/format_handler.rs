use std::path::Path;

use crate::{
    common::Verbosity,
    graphics::Graphic,
    processors::{
        image::format_handlers::Error,
        ConfigStatus
    },
    settings::Config
};

pub trait FormatHandler: Verbosity {
    fn name(&self) -> &'static str;
    fn extensions(&self) -> &[&str];
    fn setup(&self, config: &mut Config) -> Result<ConfigStatus, Error>;
    fn process(&self, source_file_path: &Path, output_folder_path: &Path, config: &Config) -> Result<Graphic, Error>;
}
