use colored::Colorize;

use crate::{
    common::Verbosity,
    processors::{
        ConfigStatus,
        data::AtlasData,
        Processor,
        State
    },
    settings::{
        Config,
        ProcessorConfig
    }
};

pub struct DataProcessor {
    verbose: bool,
    prettify_output: bool
}

impl DataProcessor {
    pub fn new() -> Self {
        Self {
            verbose: false,
            prettify_output: false
        }
    }
}

impl Processor for DataProcessor {
    fn name(&self) -> &str {
        "Data"
    }

    fn retrieve_processor_config<'a>(&self, config: &'a Config) -> &'a dyn ProcessorConfig {
        &config.data
    }

    fn setup(&mut self, config: &mut Config) -> ConfigStatus {
        self.prettify_output = config.data.prettify || config.prettify_json;
        ConfigStatus::NotModified
    }

    fn execute(&self, state: &mut State) {
        infoln!(block, "Processing data");
        let mut atlas_data = AtlasData::new();

        let cache = match &state.cache {
            Some(c) => c,
            None => panic!("Cache isn't available.")
        };

        infoln!(block, "Gathering graphics' data entries");

        for entry in cache.files.values() {
            match entry.borrow().location.file_stem() {
                Some(location_stem) => {
                    atlas_data.graphics.insert(location_stem.to_str().unwrap().to_owned(), entry.borrow().data.clone());
                },
                None => {
                    panic!("File stem not found at location '{}'", entry.borrow().location.display());
                }
            }
        }

        infoln!(last, "{}", "Done".green());

        let output_atlas_data_path = state
            .config
            .cache
            .atlas_path()
            .join(
               if state.config.output_name.is_empty() {
                   format!("{}.data.json", Config::default_output_name())
               } else {
                   format!("{}.data.json", state.config.output_name)
               }
            );

        if self.prettify_output {
            infoln!("Exporting prettified data to file");
        } else {
            infoln!("Exporting data to file");
        }

        traceln!("At {}", output_atlas_data_path.display().to_string().bold());

        if self.prettify_output {
            atlas_data.save_pretty_to_path(output_atlas_data_path).unwrap();
        } else {
            atlas_data.save_to_path(output_atlas_data_path).unwrap();
        }

        infoln!(last, "{}", "Done".green());
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
