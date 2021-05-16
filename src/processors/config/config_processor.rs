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
        ConfigStatus,
        Processor,
        State
    },
    settings::Config
};

pub struct ConfigProcessor {
}

impl Processor for ConfigProcessor {
    fn name(&self) -> &str {
        "Config"
    }

    fn setup(&mut self, config: &mut Config) -> ConfigStatus {
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
                    panic!("Expected a directory at output path '{}'.", output_pathbuf.display());
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
                        panic!("{}", io_error);
                    }
                }
            }
        }

        // output name
        if config.output_name.is_empty() {
            config.output_name = Config::default_output_name();
            config_status = ConfigStatus::Modified;
        }

        config_status
    }

    fn execute(&self, _state: &mut State) {
        // there is nothing to do at this phase
    }
}

impl ConfigProcessor {
    pub fn new() -> Self {
        ConfigProcessor {
        }
    }
}
