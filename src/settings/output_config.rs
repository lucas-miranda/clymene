use crate::{
    common::Verbosity,
    log::Logger,
    settings::{ConfigLoggerStatus, ProcessorConfig},
};
use serde::{Deserialize, Serialize};

const DEFAULT_NAME: &str = "atlas";
const DEFAULT_FOLDER_PATH: &str = "output";

#[derive(Serialize, Deserialize)]
pub struct OutputConfig {
    #[serde(default)]
    pub verbose: bool,

    #[serde(default = "OutputConfig::default_name")]
    pub name: String,

    #[serde(default = "OutputConfig::default_path")]
    pub path: String,
}

impl OutputConfig {
    pub fn default_name() -> String {
        DEFAULT_NAME.to_string()
    }

    pub fn default_path() -> String {
        DEFAULT_FOLDER_PATH.to_string()
    }

    pub fn name_or_default(&self) -> &str {
        if self.name.is_empty() {
            DEFAULT_NAME
        } else {
            &self.name
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            name: OutputConfig::default_name(),
            path: OutputConfig::default_path(),
        }
    }
}

impl ProcessorConfig for OutputConfig {
    fn configure_logger(&self, logger: &mut Logger, parent_logger_status: &ConfigLoggerStatus) {
        let logger_status = ConfigLoggerStatus {
            verbose: self.is_verbose() || parent_logger_status.verbose,
        };

        if logger_status.verbose {
            logger.register_module("modes::generator::processors::output", true);
        }
    }
}

impl Verbosity for OutputConfig {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
