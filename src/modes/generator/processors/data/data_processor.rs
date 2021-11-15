use colored::Colorize;
use std::fs;

use crate::{
    common::Verbosity,
    modes::generator::processors::{
        output::{self, OutputFile},
        ConfigStatus, Processor, State,
    },
    settings::{Config, ProcessorConfig},
    util::{self, Timer},
};

use super::AtlasData;

pub struct DataProcessor {
    verbose: bool,
}

impl DataProcessor {
    pub fn new() -> Self {
        Self { verbose: false }
    }
}

impl Processor for DataProcessor {
    fn name(&self) -> &str {
        "Data"
    }

    fn retrieve_processor_config<'a>(&self, config: &'a Config) -> Option<&'a dyn ProcessorConfig> {
        Some(&config.data)
    }

    fn setup(&mut self, state: &mut State) -> ConfigStatus {
        let mut c = state
            .config
            .try_write()
            .expect("Can't retrieve a write lock");

        c.data.prettify = c.data.prettify || c.prettify;
        ConfigStatus::NotModified
    }

    fn execute(&mut self, state: &mut State) {
        let c = state.config.try_read().expect("Can't retrieve a read lock");

        infoln!(block, "Processing data");
        let total_timer = Timer::start();
        let mut atlas_data = AtlasData::new();

        let cache = match &state.cache {
            Some(cache) => {
                if cache.is_updated()
                    && cache.meta.generation_metadata().data.prettified == c.data.prettify
                {
                    // output file
                    let output_atlas_data_path = c
                        .cache
                        .atlas_path()
                        .join(format!("{}.data.json", c.output.name_or_default()));

                    let output_file = OutputFile::new(output_atlas_data_path);

                    match state.output.register_file(output_file) {
                        Ok(()) => {
                            infoln!(last, "{}", "Already Updated".green());
                            return;
                        }
                        Err(e) => match e {
                            output::Error::FileExpected => {
                                infoln!("Output file not found, regenerating it")
                            }
                            _ => panic!("{}", e),
                        },
                    }
                }

                cache
            }
            None => panic!("Cache isn't available"),
        };

        infoln!(block, "Gathering graphics' data entries");
        let gathering_graphics_timer = Timer::start();

        for entry in cache.files.values() {
            match entry.borrow().location.file_stem() {
                Some(location_stem) => {
                    atlas_data.graphics.insert(
                        location_stem.to_str().unwrap().to_owned(),
                        entry.borrow().data.clone(),
                    );
                }
                None => {
                    panic!(
                        "File stem not found at location '{}'",
                        entry.borrow().location.display()
                    );
                }
            }
        }

        doneln_with_timer!(gathering_graphics_timer);

        let output_atlas_data_path = c
            .cache
            .atlas_path()
            .join(format!("{}.data.json", c.output.name_or_default()));

        if c.data.prettify {
            infoln!("Exporting prettified data to file");
        } else {
            infoln!("Exporting data to file");
        }

        traceln!("At {}", output_atlas_data_path.display().to_string().bold());

        // remove file at path
        if output_atlas_data_path.exists() {
            fs::remove_file(&output_atlas_data_path).unwrap();

            // wait until file is removed, if exists
            util::wait_until(|| !output_atlas_data_path.exists());
        }

        if c.data.prettify {
            atlas_data
                .save_pretty_to_path(&output_atlas_data_path)
                .unwrap();
        } else {
            atlas_data.save_to_path(&output_atlas_data_path).unwrap();
        }

        // wait until files are written
        util::wait_until(|| output_atlas_data_path.exists());

        // output
        let output_file = OutputFile::new(output_atlas_data_path);
        state.output.register_file(output_file).unwrap();

        doneln_with_timer!(total_timer)
    }
}

impl Verbosity for DataProcessor {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
