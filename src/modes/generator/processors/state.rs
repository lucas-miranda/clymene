use super::{
    cache::{Cache, CacheMetadata, DataOutputMetadata, GenerationMetadata, ImageOutputMetadata},
    image::GraphicOutput,
    output::Output,
};

use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use crate::{modes::generator::GeneratorModeArgs, settings::Config};

pub struct State<'a> {
    pub config: Arc<RwLock<Config>>,
    pub cache: Option<Cache>,
    pub graphic_output: GraphicOutput,
    pub output: Output,
    args: &'a GeneratorModeArgs,
}

impl<'a> State<'a> {
    pub fn new(config: Config, args: &GeneratorModeArgs) -> State<'_> {
        let output = Output::new(config.packer.atlas_size, config.packer.atlas_size);

        State {
            config: Arc::new(RwLock::new(config)),
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
        let c = self.config.try_read().expect("Can't retrieve a read lock");

        let source_directory_path = PathBuf::from(&c.image.input_path);
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
                prettified: c.data.prettify,
            },
        })
    }
}
