use serde::{Deserialize, Serialize};

use crate::{
    common::Verbosity,
    log::Logger,
    settings::{ConfigLoggerStatus, ProcessorConfig},
};

#[derive(Default, Serialize, Deserialize)]
pub struct AsepriteConfig {
    #[serde(default)]
    pub verbose: bool,

    #[serde(default)]
    pub bin_path: String,

    #[serde(default)]
    pub input_path: String,
}

impl ProcessorConfig for AsepriteConfig {
    fn configure_logger(&self, logger: &mut Logger, parent_logger_status: &ConfigLoggerStatus) {
        let logger_status = ConfigLoggerStatus {
            verbose: self.is_verbose() || parent_logger_status.verbose,
        };

        if logger_status.verbose {
            logger.register_module(
                "modes::generator::processors::image::format_handlers::aseprite_handler",
                true,
            );
        }
    }
}

impl Verbosity for AsepriteConfig {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
