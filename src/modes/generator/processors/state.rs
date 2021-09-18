use super::{cache::Cache, image::GraphicOutput, output::Output};
use crate::settings::Config;

pub struct State<'a> {
    pub config: &'a mut Config,
    pub cache: Option<Cache>,
    pub graphic_output: GraphicOutput,
    pub output: Output,
    pub force: bool,
}

impl<'a> State<'a> {
    pub fn new(config: &mut Config, force: bool) -> State<'_> {
        State {
            config,
            cache: None,
            graphic_output: GraphicOutput::new(),
            output: Output::new(),
            force,
        }
    }
}
