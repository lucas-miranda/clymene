use crate::{
    processors::{
        ConfigStatus,
        Data
    },
    settings::Config
};

pub trait Processor {
    fn name(&self) -> &str;
    fn setup(&mut self, config: &mut Config) -> ConfigStatus;
    fn execute(&self, data: &mut Data);
}
