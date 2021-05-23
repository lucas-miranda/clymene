use serde::{ 
    Deserialize, 
    Serialize 
};

use flexi_logger::LogSpecBuilder;

use crate::{
    common::Verbosity,
    settings::ProcessorConfig
};

#[derive(Default, Serialize, Deserialize)]
pub struct AsepriteConfig {
    #[serde(default)]
    pub verbose: bool,

    #[serde(default)]
    pub bin_path: String,

    #[serde(default)]
    pub input_path: String,
}

impl ProcessorConfig for AsepriteConfig {
    fn configure_logger(&self, builder: &mut LogSpecBuilder) {
        if self.is_verbose() {
            builder.module("raven::processors::image::format_handlers::aseprite_handler", log::LevelFilter::Trace);
        }
    }
}

impl Verbosity for AsepriteConfig {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
