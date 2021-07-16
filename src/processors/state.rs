use crate::{
    processors::{
        cache::Cache,
        image::GraphicOutput
    },
    settings::Config
};

pub struct State<'a> {
    pub config: &'a mut Config,
    pub cache: Option<Cache>,
    pub graphic_output: GraphicOutput,
    pub force: bool
}

impl<'a> State<'a> {
    pub fn new(config: &mut Config, force: bool) -> State<'_> {
        State {
            config,
            cache: None,
            graphic_output: GraphicOutput::new(),
            force
        }
    }
}
