use crate::{
    processors::{
        ConfigStatus,
        State
    },
    settings::Config
};

pub trait Processor {
    fn name(&self) -> &str;
    fn setup(&mut self, config: &mut Config) -> ConfigStatus;
    fn execute(&self, state: &mut State);
}
