use super::{cache::Cache, image::GraphicOutput, output::Output};

use crate::{modes::generator::GeneratorModeArgs, settings::Config};

pub struct State<'a> {
    pub config: &'a mut Config,
    pub cache: Option<Cache>,
    pub graphic_output: GraphicOutput,
    pub output: Output,
    args: &'a GeneratorModeArgs,
}

impl<'a> State<'a> {
    pub fn new<'c>(config: &'c mut Config, args: &'c GeneratorModeArgs) -> State<'c> {
        State {
            config,
            cache: None,
            graphic_output: GraphicOutput::new(),
            output: Output::new(),
            args,
        }
    }

    pub fn args(&self) -> &GeneratorModeArgs {
        self.args
    }
}
