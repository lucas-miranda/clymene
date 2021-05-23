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
pub struct PackerConfig {
    #[serde(default)]
    pub verbose: bool,

    #[serde(default)]
    pub atlas_size: u32,

    #[serde(default)]
    pub optimize: bool,

    #[serde(default)]
    pub force: bool
}

impl ProcessorConfig for PackerConfig {
    fn configure_logger(&self, builder: &mut LogSpecBuilder) {
        if self.is_verbose() {
            builder.module("raven::processors::packer", log::LevelFilter::Trace);
        }
    }
}

impl Verbosity for PackerConfig {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}

