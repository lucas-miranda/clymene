use std::path::Path;

use crate::{
    common::Verbosity, graphics::Graphic, modes::generator::processors::ConfigStatus,
    settings::Config,
};

pub trait FormatProcessor {
    fn setup(&self, config: &mut Config) -> eyre::Result<ConfigStatus>;
    fn process(
        &self,
        source_file_path: &Path,
        output_dir_path: &Path,
        config: &Config,
    ) -> eyre::Result<Graphic>;
}

pub trait FormatHandler: FormatProcessor + Verbosity {
    fn name(&self) -> &'static str;
    fn extensions(&self) -> &[&str];
}
