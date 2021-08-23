use crate::{common::Verbosity, log::Logger, settings::ConfigLoggerStatus};

pub trait ProcessorConfig: Verbosity {
    fn configure_logger(&self, logger: &mut Logger, parent_logger_status: &ConfigLoggerStatus);
}
