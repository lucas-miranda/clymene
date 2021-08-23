use crate::{
    common::Verbosity,
    processors::{ConfigStatus, State},
    settings::{Config, ProcessorConfig},
};

pub trait Processor: Verbosity {
    fn name(&self) -> &str;
    fn retrieve_processor_config<'a>(&self, config: &'a Config) -> &'a dyn ProcessorConfig;
    fn setup(&mut self, config: &mut Config) -> ConfigStatus;
    fn execute(&self, state: &mut State);
}
