use serde::{ 
    Deserialize, 
    Serialize 
};

use crate::{
    common::Verbosity,
    log::Logger,
    settings::{
        ConfigLoggerStatus,
        ProcessorConfig
    }
};

#[derive(Default, Serialize, Deserialize)]
pub struct DataConfig {
    #[serde(default)]
    pub verbose: bool,

    #[serde(default)]
    pub prettify: bool
}

impl ProcessorConfig for DataConfig {
    fn configure_logger(&self, logger: &mut Logger, parent_logger_status: &ConfigLoggerStatus) {
        let logger_status = ConfigLoggerStatus {
            verbose: self.is_verbose() || parent_logger_status.verbose
        };

        if logger_status.verbose {
            logger.register_module("processors::data", true);
        }
    }
}

impl Verbosity for DataConfig {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
