use crate::{
    common::Verbosity,
    modes::generator::processors::{ConfigStatus, Processor, State},
    settings::{Config, OutputConfig, ProcessorConfig},
    util,
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

    fn setup(&mut self, state: &mut State) -> ConfigStatus {
        let mut config_status = ConfigStatus::NotModified;
        let mut c = state
            .config
            .try_write()
            .expect("Can't retrieve a write lock");

        infoln!(block, "Output");

        let output_path = if c.output.path.is_empty() {
            let p = OutputConfig::default_path();
            warnln!("Output directory path is empty, default value will be used.");
            let path = PathBuf::from(&p);
            c.output.path = p;
            config_status = ConfigStatus::Modified;
            path
        } else {
            traceln!("Using provided directory path");
            PathBuf::from(&c.output.path)
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
            Err(io_error) => match &io_error.kind() {
                io::ErrorKind::NotFound => {
                    warnln!("Directory doesn't seems to exist");

                    infoln!(
                        entry: decorator::Entry::None,
                        "It'll be created right now..."
                    );

                    fs::create_dir_all(&output_pathbuf).unwrap();
                    util::wait_until(|| output_pathbuf.exists());

                    infoln!(last, "{}", "Created".green());
                }
                _ => {
                    panic!("{}", io_error);
                }
            },
        }

        doneln!();
        config_status
    }

    fn execute(&mut self, state: &mut State) {
        let c = state.config.try_read().expect("Can't retrieve a read lock");
        infoln!(block, "Output");
        let output = PathBuf::from(&c.output.path);
        infoln!("To {}", output.display().to_string().bold());

        if !output.is_dir() {
            panic!(
                "Output path '{}' doesn't exists or isn't a valid directory.",
                output.display()
            );
        }

        for output_file in state.output.files() {
            match output_file.path().metadata() {
                Ok(metadata) => {
                    if !metadata.is_file() {
                        infoln!(block, "{}", output_file.path.display().to_string().bold());
                        errorln!(last, "Isn't a valid file");
                        continue;
                    }

                    let filename = output_file.path.file_name().unwrap();
                    infoln!(block, "{}", filename.to_str().unwrap().bold());

                    let output_filepath = output.join(filename);
                    if let Ok(output_metadata) = output_filepath.metadata() {
                        if let Ok(output_modtime) = output_metadata.modified() {
                            let modtime = metadata.modified().unwrap();

                            if modtime <= output_modtime {
                                infoln!(last, "{}", "Unchanged".blue().bold());
                                continue;
                            }
                        }
                    }

                    if let Some(stats) = output_file.stats() {
                        stats.display_stats();
                    }

                    match fs::copy(&output_file.path, output_filepath) {
                        Ok(_) => infoln!(last, "{}", "Generated".green().bold()),
                        Err(err) => errorln!(last, "{}  {}", "Error".red().bold(), err),
                    }
                }
                Err(e) => {
                    infoln!(block, "{}", output_file.path.display().to_string().bold());
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
