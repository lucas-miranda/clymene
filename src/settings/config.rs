use std::{
    fs::{
        File,
        OpenOptions
    },
    io::{
        self,
        BufReader,
        BufWriter,
        prelude::*
    },
    path::Path
};

use serde::{ 
    Deserialize, 
    Serialize 
};

use crate::settings::{
    CacheConfig,
    DataConfig,
    ImageConfig,
    LoadError,
    SaveError,
    PackerConfig
};

const DEFAULT_OUTPUT_NAME: &str = "atlas";
const DEFAULT_OUTPUT_FOLDER_PATH: &str = "output";

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub verbose: bool,

    #[serde(default = "Config::default_output_name")]
    pub output_name: String,

    #[serde(default = "Config::default_output_path")]
    pub output_path: String,

    #[serde(default)]
    pub prettify_json: bool,

    #[serde(default)]
    pub cache: CacheConfig,

    #[serde(default)]
    pub packer: PackerConfig,

    #[serde(default)]
    pub image: ImageConfig,

    #[serde(default)]
    pub data: DataConfig,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            verbose: false,
            output_name: Config::default_output_name(),
            output_path: Config::default_output_path(),
            prettify_json: false,
            cache: CacheConfig::default(),
            packer: PackerConfig::default(),
            image: ImageConfig::default(),
            data: DataConfig::default()
        }
    }
}

impl Config {
    pub fn load(file: &File) -> Result<Config, LoadError> {
        let mut contents = String::new();

        BufReader::new(file)
                  .read_to_string(&mut contents)
                  .unwrap();

        toml::from_str(&contents)
             .map_err(LoadError::Deserialize)
    }

    pub fn load_from_path<P: AsRef<Path>>(filepath: P) -> Result<Config, LoadError> {
        let file = match OpenOptions::new().read(true).write(true).append(false).open(&filepath) {
            Ok(f) => Ok(f),
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => Err(LoadError::FileNotFound(filepath.as_ref().to_owned())),
                    _ => panic!("{}", e)
                }
            }
        }?;

        Self::load(&file)
    }

    pub fn default_output_name() -> String {
        DEFAULT_OUTPUT_NAME.to_string()
    }

    pub fn default_output_path() -> String {
        DEFAULT_OUTPUT_FOLDER_PATH.to_string()
    }

    pub fn save(&self, file: &File) -> Result<(), SaveError> {
        let toml_data = toml::to_string(&self)
                             .map_err(SaveError::Serialize)?;

        let mut buffer = BufWriter::new(file);
        buffer.write_all(toml_data.as_bytes()).unwrap();

        Ok(())
    }

    pub fn save_to_path<P: AsRef<Path>>(&self, filepath: P) -> Result<(), SaveError> {
        let file = OpenOptions::new()
                               .write(true)
                               .append(false)
                               .create(true)
                               .open(filepath)
                               .unwrap();

        self.save(&file)
    }
}
