use crate::{
    common::Verbosity,
    settings::{Config, ProcessorConfig},
};

use super::{ConfigStatus, State};

pub trait Processor: Verbosity {
    fn name(&self) -> &str;
    fn retrieve_processor_config<'a>(&self, config: &'a Config) -> Option<&'a dyn ProcessorConfig>;
    fn setup(&mut self, state: &mut State) -> ConfigStatus;
    fn execute(&self, state: &mut State);
}
