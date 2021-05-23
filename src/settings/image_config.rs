use std::path::PathBuf;

use flexi_logger::LogSpecBuilder;

use serde::{ 
    Deserialize, 
    Serialize 
};

use crate::{
    common::Verbosity,
    settings::{
        AsepriteConfig,
        ProcessorConfig
    }
};

#[derive(Default, Serialize, Deserialize)]
pub struct ImageConfig {
    #[serde(default)]
    pub verbose: bool,

    #[serde(default)]
    pub input_path: String,

    #[serde(skip)]
    pub output_path: PathBuf,

    #[serde(default)]
    pub aseprite: AsepriteConfig
}

impl ImageConfig {
    pub fn source_path(&self) -> PathBuf {
        PathBuf::from(&self.input_path)
    }
}

impl ProcessorConfig for ImageConfig {
    fn configure_logger(&self, builder: &mut LogSpecBuilder) {
        if self.is_verbose() {
            builder.module("raven::processors::image", log::LevelFilter::Trace);
            return;
        }

        self.aseprite.configure_logger(builder);
    }
}

impl Verbosity for ImageConfig {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
