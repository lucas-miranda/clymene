use colored::Colorize;
use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom},
    path::Path,
};
use tree_decorator::decorator;

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

const ASEPRITE_FILE_MAGIC_NUMBER: [u8; 2] = [0xE0, 0xA5];

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

    fn validate_file(&self, source_file_path: &Path) -> Result<(), Error> {
        match source_file_path.metadata() {
            Ok(metadata) => {
                if !metadata.is_file() {
                    return Err(Error::FileExpected(source_file_path.to_path_buf()));
                }

                // check magic number section
                let mut file = OpenOptions::new()
                    .read(true)
                    .open(&source_file_path)
                    .unwrap();

                file.seek(SeekFrom::Start(4)).unwrap(); // seek to magic number

                let mut buffer = [0u8; 2];
                file.read_exact(&mut buffer).unwrap();

                if buffer[..] != ASEPRITE_FILE_MAGIC_NUMBER[..] {
                    // magic number doesn't match
                    return Err(Error::WrongFileType);
                }
            }
            Err(e) => {
                panic!("{}", e)
            }
        }

        Ok(())
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
        traceln!(
            entry: decorator::Entry::None,
            "Source filepath: {}",
            source_file_path.display().to_string().bold()
        );

        self.validate_file(source_file_path)?;

        // verify output directory
        traceln!(
            entry: decorator::Entry::None,
            "Output dir: {}",
            output_dir_path.display().to_string().bold()
        );

        if !output_dir_path.is_dir() {
            return Err(Error::DirectoryExpected);
        }

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
