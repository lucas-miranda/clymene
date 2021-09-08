use std::path::Path;

use crate::{
    common::Verbosity, graphics::Graphic, modes::generator::processors::ConfigStatus,
    settings::Config,
};

use super::Error;

pub trait FormatProcessor {
    fn setup(&self, config: &mut Config) -> Result<ConfigStatus, Error>;
    fn process(
        &self,
        source_file_path: &Path,
        output_dir_path: &Path,
        config: &Config,
    ) -> Result<Graphic, Error>;
}

pub trait FormatHandler: FormatProcessor + Verbosity {
    fn name(&self) -> &'static str;
    fn extensions(&self) -> &[&str];
}
