use std::{
    fs,
    io,
    path::PathBuf
};

use colored::Colorize;

use log::{
    info,
    trace,
    warn
};

use crate::{
    processors::{
        config,
        ConfigStatus,
        Data,
        Error,
        Processor
    },
    settings::Config
};

pub struct ConfigProcessor {
}

impl Processor for ConfigProcessor {
    fn setup(&mut self, config: &mut Config) -> Result<ConfigStatus, Error> {
        let mut config_status = ConfigStatus::NotModified;
        let mut output_path = config.output_path.clone();

        if output_path.is_empty() {
            output_path = Config::default_output_path();

            warn!(
                "{}  Output dir path is empty, default value '{}' will be used.", 
                "Raven".bold(), 
                output_path
            );

            config_status = ConfigStatus::Modified;
        }

        // handle output folder path
        let output_pathbuf = PathBuf::from(&output_path);

        match output_pathbuf.metadata() {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    return Err(config::Error::InvalidOutputPath(output_pathbuf.display().to_string()).into());
                }
            },
            Err(io_error) => {
                match &io_error.kind() {
                    io::ErrorKind::NotFound => {
                        trace!(
                            "{}  Output dir path '{}' doesn't seems to exist, it'll be created right now...", 
                            "Raven".bold(), 
                            output_pathbuf.display()
                        );

                        fs::create_dir_all(&output_pathbuf).unwrap();

                        // wait until folder is created
                        while !output_pathbuf.exists() {
                            std::thread::sleep(std::time::Duration::from_millis(10u64));
                        }

                        info!("{}  Output folder created!", "Raven".bold());
                    },
                    _ => {
                        return Err(config::Error::IO(io_error).into());
                    }
                }
            }
        }

        // output name
        if config.output_name.is_empty() {
            config.output_name = Config::default_output_name();
            config_status = ConfigStatus::Modified;
        }

        Ok(config_status)
    }

    fn execute(&self, _data: &mut Data) -> Result<(), Error> {
        // there is nothing to do at this phase
        Ok(())
    }
}

impl ConfigProcessor {
    pub fn new() -> Self {
        ConfigProcessor {
        }
    }
}
