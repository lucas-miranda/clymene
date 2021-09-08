mod logger;
mod logger_module_entry;

#[macro_use]
pub mod macros;

#[macro_use]
pub mod specialized_macros;

use crate::{
    settings::{Config, ConfigLoggerStatus, DisplayKind, ProcessorConfig},
    GlobalArgs,
};

pub use logger::Logger;
pub use logger_module_entry::LoggerModuleEntry;
use tree_decorator::{DecoratorBuilder, StandardDecorator};

static mut LOGGER: Option<Logger> = None;

pub fn logger<'a>() -> &'a Option<Logger> {
    unsafe { &LOGGER }
}

pub fn initialize_logger(config: &mut Config, global_args: &GlobalArgs) {
    let mut logger = Logger::default();

    if global_args.debug {
        logger.debug(true);
    }

    if global_args.verbose {
        logger.verbose(true);
    }

    let logger_status = ConfigLoggerStatus {
        verbose: logger.is_verbose(),
    };

    config.configure_logger(&mut logger, &logger_status);

    if logger.is_verbose() {
        config.image.display = DisplayKind::Detailed;
    }

    // tree decorator
    DecoratorBuilder::with(StandardDecorator::new(2)).build();

    unsafe {
        LOGGER = Some(logger);
    }
}
