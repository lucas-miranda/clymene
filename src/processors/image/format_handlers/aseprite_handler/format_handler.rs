use std::path::Path;

use crate::{
    common::Verbosity,
    graphics::Graphic,
    processors::{
        image::format_handlers::{Error, FormatHandler, FormatProcessor},
        ConfigStatus,
    },
    settings::Config,
};

use super::{AsepriteProcessor, CommandProcessor, RawFileProcessor};

pub struct AsepriteFormatHandler {
    verbose: bool,
    processor: Box<dyn FormatProcessor>,
}

impl AsepriteFormatHandler {
    pub fn new(processor: AsepriteProcessor) -> Self {
        Self {
            verbose: false,
            processor: match processor {
                AsepriteProcessor::Command => Box::new(CommandProcessor::default()),
                AsepriteProcessor::RawFile => Box::new(RawFileProcessor::default()),
            },
        }
    }
}

impl FormatHandler for AsepriteFormatHandler {
    fn name(&self) -> &'static str {
        "Aseprite"
    }

    fn extensions(&self) -> &[&str] {
        &["ase", "aseprite"]
    }
}

impl FormatProcessor for AsepriteFormatHandler {
    fn setup(&self, config: &mut Config) -> Result<ConfigStatus, Error> {
        self.processor.setup(config)
    }

    fn process(
        &self,
        source_file_path: &Path,
        output_dir_path: &Path,
        config: &Config,
    ) -> Result<Graphic, Error> {
        self.processor
            .process(source_file_path, output_dir_path, config)
    }
}

impl Verbosity for AsepriteFormatHandler {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
