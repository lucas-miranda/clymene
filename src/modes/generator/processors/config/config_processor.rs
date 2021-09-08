use std::{fs, io, path::PathBuf};

use colored::Colorize;
use tree_decorator::{close_tree_item, decorator};

use crate::{
    common::Verbosity,
    modes::generator::processors::{ConfigStatus, Processor, State},
    settings::{Config, ProcessorConfig},
};

pub struct ConfigProcessor {
    verbose: bool,
}

impl ConfigProcessor {
    pub fn new() -> Self {
        ConfigProcessor { verbose: false }
    }
}

impl Processor for ConfigProcessor {
    fn name(&self) -> &str {
        "Config"
    }

    fn retrieve_processor_config<'a>(&self, config: &'a Config) -> &'a dyn ProcessorConfig {
        config
    }

    fn setup(&mut self, config: &mut Config) -> ConfigStatus {
        let mut config_status = ConfigStatus::NotModified;

        infoln!(
            block,
            "Checking {} config",
            env!("CARGO_PKG_NAME").bold().magenta()
        );

        traceln!(block, "Output path");

        let output_path = if config.output_path.is_empty() {
            let p = Config::default_output_path();
            warnln!(
                "Output directory path is empty, default value '{}' will be used.",
                p
            );
            let path = PathBuf::from(&p);
            config.output_path = p;
            config_status = ConfigStatus::Modified;
            path
        } else {
            traceln!(
                "Using provided output directory path {}",
                config.output_path.bold()
            );
            PathBuf::from(&config.output_path)
        };

        // handle output folder path
        let output_pathbuf = PathBuf::from(&output_path);

        match output_pathbuf.metadata() {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    panic!(
                        "Expected a directory at output path '{}'.",
                        output_pathbuf.display()
                    );
                } else {
                    traceln!(last, "{}", "Found".green());
                }
            }
            Err(io_error) => {
                match &io_error.kind() {
                    io::ErrorKind::NotFound => {
                        warnln!(
                            block,
                            "Output directory path {} doesn't seems to exist",
                            output_pathbuf.display().to_string().bold()
                        );
                        infoln!(
                            entry: decorator::Entry::None,
                            "It'll be created right now..."
                        );

                        fs::create_dir_all(&output_pathbuf).unwrap();

                        // wait until folder is created
                        while !output_pathbuf.exists() {
                            std::thread::sleep(std::time::Duration::from_millis(10u64));
                        }

                        infoln!(last, "{}", "Ok".green());
                    }
                    _ => {
                        panic!("{}", io_error);
                    }
                }

                close_tree_item!();
            }
        }

        // output name
        if config.output_name.is_empty() {
            config.output_name = Config::default_output_name();
            config_status = ConfigStatus::Modified;
        }

        infoln!(last, "{}", "Ok".green());

        config_status
    }

    fn execute(&self, _state: &mut State) {
        // there is nothing to do at this phase
    }
}

impl Verbosity for ConfigProcessor {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
