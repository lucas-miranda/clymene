use std::path::PathBuf;

use serde::{ 
    Deserialize, 
    Serialize 
};

const IMAGES_FOLDER_NAME: &str = "images";
const ATLAS_FOLDER_NAME: &str = "atlas";

#[derive(Serialize, Deserialize)]
pub struct CacheConfig {
    #[serde(default)]
    pub verbose: bool,

    #[serde(default)]
    pub path: String,

    #[serde(default)]
    pub identifier: String,
}

impl Default for CacheConfig {
    fn default() -> CacheConfig {
        CacheConfig {
            verbose: false,
            path: String::new(),
            identifier: String::new()
        }
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
