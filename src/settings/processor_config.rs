use flexi_logger::LogSpecBuilder;
use crate::common::Verbosity;

pub trait ProcessorConfig: Verbosity {
    fn configure_logger(&self, builder: &mut LogSpecBuilder);
}
