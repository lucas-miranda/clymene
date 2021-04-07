use crate::{
    processors::{
        ConfigStatus,
        Data,
        Error
    },
    settings::Config
};

pub trait Processor {
    fn setup(&mut self, config: &mut Config) -> Result<ConfigStatus, Error>;
    fn execute(&self, data: &mut Data) -> Result<(), Error>;
}
