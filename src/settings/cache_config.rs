use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    common::Verbosity,
    log::Logger,
    settings::{ConfigLoggerStatus, ProcessorConfig},
};

const IMAGES_DIR_NAME: &str = "images";
const ATLAS_DIR_NAME: &str = "atlas";

#[derive(Default, Serialize, Deserialize)]
pub struct CacheConfig {
    #[serde(default)]
    pub verbose: bool,

    #[serde(default)]
    pub path: String,

    #[serde(default)]
    pub identifier: String,
}

impl ProcessorConfig for CacheConfig {
    fn configure_logger(&self, logger: &mut Logger, parent_logger_status: &ConfigLoggerStatus) {
        let logger_status = ConfigLoggerStatus {
            verbose: self.is_verbose() || parent_logger_status.verbose,
        };

        if logger_status.verbose {
            logger.register_module("modes::generator::processors::cache", true);
        }
    }
}

impl Verbosity for CacheConfig {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}

impl CacheConfig {
    pub fn entry_path(&self) -> PathBuf {
        PathBuf::from(&self.path).join(&self.identifier)
    }

    pub fn images_path(&self) -> PathBuf {
        self.entry_path().join(IMAGES_DIR_NAME)
    }

    pub fn atlas_path(&self) -> PathBuf {
        self.entry_path().join(ATLAS_DIR_NAME)
    }
}
