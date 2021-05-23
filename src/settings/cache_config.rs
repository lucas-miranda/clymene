use std::path::PathBuf;

use serde::{ 
    Deserialize, 
    Serialize 
};

use flexi_logger::LogSpecBuilder;

use crate::{
    common::Verbosity,
    settings::ProcessorConfig
};

const IMAGES_FOLDER_NAME: &str = "images";
const ATLAS_FOLDER_NAME: &str = "atlas";

#[derive(Default, Serialize, Deserialize)]
pub struct CacheConfig {
    #[serde(default)]
    pub verbose: bool,

    #[serde(default)]
    pub path: String,

    #[serde(default)]
    pub identifier: String,
}

impl ProcessorConfig for CacheConfig {
    fn configure_logger(&self, builder: &mut LogSpecBuilder) {
        if self.is_verbose() {
            builder.module("raven::processors::cache", log::LevelFilter::Trace);
        }
    }
}

impl Verbosity for CacheConfig {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}

impl CacheConfig {
    pub fn root_path(&self) -> PathBuf {
        PathBuf::from(&self.path).join(&self.identifier)
    }

    pub fn images_path(&self) -> PathBuf {
        self.root_path().join(IMAGES_FOLDER_NAME)
    }

    pub fn atlas_path(&self) -> PathBuf {
        self.root_path().join(ATLAS_FOLDER_NAME)
    }
}
