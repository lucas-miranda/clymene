use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct CacheMetadata {
    version: String,
    generation: GenerationMetadata,
}

impl CacheMetadata {
    pub fn new(generation: GenerationMetadata) -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            generation,
        }
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn generation_metadata(&self) -> &GenerationMetadata {
        &self.generation
    }
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct GenerationMetadata {
    pub image: ImageOutputMetadata,
    pub data: DataOutputMetadata,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct ImageOutputMetadata {
    pub source_directory_modtime: SystemTime,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct DataOutputMetadata {
    pub prettified: bool,
}
