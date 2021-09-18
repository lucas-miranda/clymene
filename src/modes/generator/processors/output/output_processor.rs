use crate::{
    common::Verbosity,
    modes::generator::processors::{ConfigStatus, Processor, State},
    settings::{Config, ProcessorConfig},
};
use colored::Colorize;
use std::{fs, io, path::PathBuf};
use tree_decorator::decorator;

#[derive(Default)]
pub struct OutputProcessor {
    verbose: bool,
}

impl Processor for OutputProcessor {
    fn name(&self) -> &str {
        "Output"
    }

    fn retrieve_processor_config<'a>(
        &self,
        _config: &'a Config,
    ) -> Option<&'a dyn ProcessorConfig> {
        None
    }

    fn setup(&mut self, config: &mut Config) -> ConfigStatus {
        let mut config_status = ConfigStatus::NotModified;
        traceln!(block, "Output");

        let output_path = if config.output_path.is_empty() {
            let p = Config::default_output_path();
            warnln!("Output directory path is empty, default value will be used.");
            let path = PathBuf::from(&p);
            config.output_path = p;
            config_status = ConfigStatus::Modified;
            path
        } else {
            traceln!("Using provided directory path");
            PathBuf::from(&config.output_path)
        };

        // handle output folder path
        let output_pathbuf = PathBuf::from(&output_path);

        infoln!(block, "At {}", output_path.display().to_string().bold());

        match output_pathbuf.metadata() {
            Ok(metadata) => {
                if metadata.is_dir() {
                    infoln!(last, "{}", "Found".green());
                } else {
                    panic!(
                        "Expected a directory at output path '{}'.",
                        output_pathbuf.display()
                    );
                }
            }
            Err(io_error) => {
                match &io_error.kind() {
                    io::ErrorKind::NotFound => {
                        warnln!("Directory doesn't seems to exist");

                        infoln!(
                            entry: decorator::Entry::None,
                            "It'll be created right now..."
                        );

                        fs::create_dir_all(&output_pathbuf).unwrap();

                        // wait until folder is created
                        while !output_pathbuf.exists() {
                            std::thread::sleep(std::time::Duration::from_millis(10u64));
                        }

                        infoln!(last, "{}", "Created".green());
                    }
                    _ => {
                        panic!("{}", io_error);
                    }
                }
            }
        }

        doneln!();
        config_status
    }

    fn execute(&self, state: &mut State) {
        infoln!(block, "Output");
        let output = PathBuf::from(&state.config.output_path);
        infoln!("To {}", output.display().to_string().bold());

        if !output.is_dir() {
            panic!(
                "Output path '{}' doesn't exists or isn't a valid directory.",
                output.display()
            );
        }

        for filepath in state.output.files() {
            match filepath.metadata() {
                Ok(metadata) => {
                    if !metadata.is_file() {
                        infoln!(block, "{}", filepath.display().to_string().bold());
                        errorln!(last, "Isn't a valid file");
                        continue;
                    }

                    let filename = filepath.file_name().unwrap();
                    infoln!(block, "{}", filename.to_str().unwrap().bold());

                    let output_filepath = output.join(filename);
                    if let Ok(output_metadata) = output_filepath.metadata() {
                        if let Ok(output_modtime) = output_metadata.modified() {
                            let modtime = metadata.modified().unwrap();

                            if modtime < output_modtime {
                                infoln!(last, "{}", "Unchanged".blue().bold());
                                continue;
                            }
                        }
                    }

                    match fs::copy(&filepath, output_filepath) {
                        Ok(_) => infoln!(last, "{}", "Generated".green().bold()),
                        Err(err) => errorln!(last, "{}  {}", "Error".red().bold(), err),
                    }
                }
                Err(e) => {
                    infoln!(block, "{}", filepath.display().to_string().bold());
                    errorln!(last, "{}  {}", "Error".red().bold(), e);
                }
            }
        }

        doneln!();
    }
}

impl Verbosity for OutputProcessor {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
