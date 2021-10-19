use super::{
    cache::{Cache, CacheMetadata, DataOutputMetadata, GenerationMetadata, ImageOutputMetadata},
    image::GraphicOutput,
    output::Output,
};

use std::path::PathBuf;

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
        let output = Output::new(config.packer.atlas_size, config.packer.atlas_size);

        State {
            config,
            cache: None,
            graphic_output: GraphicOutput::new(),
            output,
            args,
        }
    }

    pub fn args(&self) -> &GeneratorModeArgs {
        self.args
    }

    pub fn create_cache_metadata(&self) -> CacheMetadata {
        let source_directory_path = PathBuf::from(&self.config.image.input_path);
        let source_directory_modtime = source_directory_path
            .metadata()
            .unwrap()
            .modified()
            .unwrap();

        CacheMetadata::new(GenerationMetadata {
            image: ImageOutputMetadata {
                source_directory_modtime,
                width: self.output.atlas_width,
                height: self.output.atlas_height,
            },
            data: DataOutputMetadata {
                prettified: self.config.data.prettify,
            },
        })
    }
}
