use std::path::PathBuf;

use serde::{ 
    Deserialize, 
    Serialize 
};

use crate::settings::AsepriteConfig;

#[derive(Serialize, Deserialize)]
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

impl Default for ImageConfig {
    fn default() -> ImageConfig {
        ImageConfig {
            verbose: false,
            input_path: String::new(),
            output_path: PathBuf::default(),
            aseprite: AsepriteConfig::default()
        }
    }
}
