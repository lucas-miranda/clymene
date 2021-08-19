use std::path::PathBuf;

use serde::{ 
    Deserialize, 
    Serialize 
};

use crate::{
    common::Verbosity,
    log::Logger,
    settings::{
        AsepriteConfig,
        ConfigLoggerStatus,
        ProcessorConfig
    }
};

#[derive(Serialize, Deserialize)]
pub struct ImageConfig {
    #[serde(default)]
    pub verbose: bool,

    #[serde(default)]
    pub input_path: String,

    #[serde(skip)]
    pub output_path: PathBuf,

    #[serde(default = "ImageConfig::default_display")]
    pub display: DisplayKind,

    #[serde(default)]
    pub aseprite: AsepriteConfig
}

impl ImageConfig {
    pub fn default_display() -> DisplayKind {
        DisplayKind::Simple
    }
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            input_path: String::default(),
            output_path: PathBuf::default(),
            display: DisplayKind::Simple,
            aseprite: AsepriteConfig::default()
        }
    }
}

impl ProcessorConfig for ImageConfig {
    fn configure_logger(&self, logger: &mut Logger, parent_logger_status: &ConfigLoggerStatus) {
        let logger_status = ConfigLoggerStatus {
            verbose: self.is_verbose() || parent_logger_status.verbose
        };

        if logger_status.verbose {
            logger.register_module("processors::image", true);
        }

        self.aseprite.configure_logger(logger, &logger_status);
    }
}

impl Verbosity for ImageConfig {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DisplayKind {
    Simple,
    List,
    Detailed
}
