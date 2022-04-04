use serde::{Deserialize, Serialize};

use crate::{
    common::Verbosity,
    log::Logger,
    settings::{ConfigLoggerStatus, PackerRetryConfig, ProcessorConfig},
};

#[derive(Serialize, Deserialize)]
pub struct PackerConfig {
    #[serde(default)]
    pub verbose: bool,

    #[serde(default)]
    pub atlas_size: u32,

    #[serde(default)]
    pub optimize: bool,

    #[serde(default)]
    pub force: bool,

    #[serde(default)]
    pub retry: PackerRetryConfig,
}

impl Default for PackerConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            atlas_size: 1024,
            optimize: true,
            force: false,
            retry: PackerRetryConfig::default(),
        }
    }
}

impl ProcessorConfig for PackerConfig {
    fn configure_logger(&self, logger: &mut Logger, parent_logger_status: &ConfigLoggerStatus) {
        let logger_status = ConfigLoggerStatus {
            verbose: self.is_verbose() || parent_logger_status.verbose,
        };

        if logger_status.verbose {
            logger.register_module("modes::generator::processors::packer", true);
        }
    }
}

impl Verbosity for PackerConfig {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
