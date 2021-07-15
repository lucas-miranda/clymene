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

use crate::{
    common::Verbosity,
    log::Logger,
    settings::{
        CacheConfig,
        ConfigLoggerStatus,
        DataConfig,
        ImageConfig,
        LoadError,
        SaveError,
        PackerConfig,
        ProcessorConfig
    }
};

const DEFAULT_OUTPUT_NAME: &str = "atlas";
const DEFAULT_OUTPUT_FOLDER_PATH: &str = "output";

#[derive(Default, Serialize, Deserialize)]
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

impl ProcessorConfig for Config {
    fn configure_logger(&self, logger: &mut Logger, parent_logger_status: &ConfigLoggerStatus) {
        let logger_status = ConfigLoggerStatus {
            verbose: self.is_verbose() || parent_logger_status.verbose
        };

        if logger_status.verbose {
            logger.verbose(true);
        }

        self.cache.configure_logger(logger, &logger_status);
        self.packer.configure_logger(logger, &logger_status);
        self.image.configure_logger(logger, &logger_status);
        self.data.configure_logger(logger, &logger_status);
    }
}

impl Verbosity for Config {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
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
