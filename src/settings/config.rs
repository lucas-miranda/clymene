use std::{
    fs::{File, OpenOptions},
    io::{self, prelude::*, BufReader, BufWriter},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::{
    common::Verbosity,
    log::Logger,
    settings::{
        CacheConfig, ConfigLoggerStatus, DataConfig, ImageConfig, LoadError, OutputConfig,
        PackerConfig, ProcessorConfig, SaveError,
    },
};

#[derive(Serialize, Deserialize, Default)]
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
    pub fn load(file: &File) -> eyre::Result<Config> {
        let mut contents = String::new();
        BufReader::new(file).read_to_string(&mut contents).unwrap();
        toml::from_str(&contents).map_err(|e| LoadError::Deserialize(e).into())
    }

    pub fn load_from_path<P: AsRef<Path>>(filepath: P) -> eyre::Result<Config> {
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
        let config_from_path = Config::load_from_path(&filepath);

        match config_from_path {
            Ok(config) => config,
            Err(e) => match e.downcast_ref::<LoadError>().unwrap() {
                LoadError::FileNotFound(path) => {
                    println!("Config file created at '{}'.", path.display());
                    let c = Config::default();
                    c.save_to_path(&path).unwrap();
                    c
                },
                _ => Err(e)
                    .map_err(|e| e.wrap_err(format!(
                        "When loading from file at '{}'",
                        filepath.as_ref().display()
                    )))
                    .unwrap(),
            }
        }
    }

    pub fn save(&self, file: &File) -> eyre::Result<()> {
        let toml_data = toml::to_string(&self).map_err(|e| eyre::Error::from(SaveError::Serialize(e)))?;

        let mut buffer = BufWriter::new(file);
        buffer.write_all(toml_data.as_bytes()).unwrap();

        Ok(())
    }

    pub fn save_to_path<P: AsRef<Path>>(&self, filepath: P) -> eyre::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .open(filepath)
            .unwrap();

        self.save(&file)
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
