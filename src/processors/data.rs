use crate::{
    processors::{
        cache::Cache,
        image::GraphicOutput
    },
    settings::Config
};

pub struct Data<'a> {
    pub config: &'a mut Config,
    pub cache: Option<Cache>,
    pub graphic_output: GraphicOutput
}

impl<'a> Data<'a> {
    pub fn new<'c>(config: &'c mut Config) -> Data<'c> {
        Data {
            config,
            cache: None,
            graphic_output: GraphicOutput::new()
        }
    }
}
