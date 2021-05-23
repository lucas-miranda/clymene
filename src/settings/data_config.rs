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
pub struct DataConfig {
    #[serde(default)]
    pub verbose: bool,

    #[serde(default)]
    pub prettify: bool
}

impl ProcessorConfig for DataConfig {
    fn configure_logger(&self, builder: &mut LogSpecBuilder) {
        if self.is_verbose() {
            builder.module("raven::processors::data", log::LevelFilter::Trace);
        }
    }
}

impl Verbosity for DataConfig {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
