use std::{
    fs::{File, OpenOptions},
    io::{self, prelude::*, BufReader, BufWriter},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    common::Verbosity,
    log::Logger,
    modes::generator::processors::cache::{
        CacheMetadata, DataOutputMetadata, GenerationMetadata, ImageOutputMetadata,
    },
    settings::{
        CacheConfig, ConfigLoggerStatus, DataConfig, ImageConfig, LoadError, OutputConfig,
        PackerConfig, ProcessorConfig, SaveError,
    },
};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub verbose: bool,

    #[serde(default)]
    pub prettify: bool,

    #[serde(default)]
    pub output: OutputConfig,

    #[serde(default)]
    pub cache: CacheConfig,

    #[serde(default)]
    pub packer: PackerConfig,

    #[serde(default)]
    pub image: ImageConfig,

    #[serde(default)]
    pub data: DataConfig,
}

impl Config {
    pub fn load(file: &File) -> Result<Config, LoadError> {
        let mut contents = String::new();
        BufReader::new(file).read_to_string(&mut contents).unwrap();
        toml::from_str(&contents).map_err(LoadError::Deserialize)
    }

    pub fn load_from_path<P: AsRef<Path>>(filepath: P) -> Result<Config, LoadError> {
        let file = match OpenOptions::new()
            .read(true)
            .write(true)
            .append(false)
            .open(&filepath)
        {
            Ok(f) => Ok(f),
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    Err(LoadError::FileNotFound(filepath.as_ref().to_owned()))
                }
                _ => panic!("{}", e),
            },
        }?;

        Self::load(&file)
    }

    pub fn load_from_path_or_default<P: AsRef<Path>>(filepath: P) -> Config {
        Config::load_from_path(&filepath).unwrap_or_else(|e| match e {
            LoadError::Deserialize(de_err) => {
                panic!(
                    "At file '{}'\nError: {:?}\nDetails: {}",
                    filepath.as_ref().display(),
                    de_err,
                    de_err.to_string()
                );
            }
            LoadError::FileNotFound(path) => {
                println!("Config file created at '{}'.", path.display());
                let c = Config::default();
                c.save_to_path(&path).unwrap();
                c
            }
        })
    }

    pub fn save(&self, file: &File) -> Result<(), SaveError> {
        let toml_data = toml::to_string(&self).map_err(SaveError::Serialize)?;

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

    pub fn cache_metadata(&self) -> CacheMetadata {
        let source_directory_path = PathBuf::from(&self.image.input_path);
        let source_directory_modtime = source_directory_path
            .metadata()
            .unwrap()
            .modified()
            .unwrap();

        CacheMetadata::new(GenerationMetadata {
            image: ImageOutputMetadata {
                source_directory_modtime,
            },
            data: DataOutputMetadata {
                prettified: self.data.prettify,
            },
        })
    }
}

impl ProcessorConfig for Config {
    fn configure_logger(&self, logger: &mut Logger, parent_logger_status: &ConfigLoggerStatus) {
        let logger_status = ConfigLoggerStatus {
            verbose: self.is_verbose() || parent_logger_status.verbose,
        };

        if logger_status.verbose {
            logger.verbose(true);
        }

        self.output.configure_logger(logger, &logger_status);
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
