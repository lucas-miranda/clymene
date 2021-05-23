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

const CACHE_ENTRY_DATA_FILENAME: &str = "data";

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
        let mut atlas_data = AtlasData::new();

        let cache = match &state.cache {
            Some(c) => c,
            None => panic!("Cache isn't available.")
        };

        //let cache_images_path = state.config.cache.images_path();
        log::info!("Gathering data entries...");
        for entry in cache.files.values() {
            match entry.borrow().location.file_stem() {
                Some(location_stem) => {
                    atlas_data.graphics.insert(location_stem.to_str().unwrap().to_owned(), entry.borrow().data.clone());
                },
                None => {
                    panic!("File stem not found at location '{}'", entry.borrow().location.display());
                }
            }

            /*
            let cache_entry_path = cache_images_path.join(&entry.borrow().location);

            // try to find entry's data file
            let dir_iter = match fs::read_dir(&cache_entry_path) {
                Ok(iter) => iter,
                Err(e) => {
                    log::error!("Can't read directory '{}', when looking for data file: {}", cache_entry_path.display(), e);
                    continue;
                }
            };

            let mut data_path = None;
            for dir_entry in dir_iter {
                match dir_entry {
                    Ok(dir_entry) => {
                        if !dir_entry.metadata().unwrap().is_file() {
                            continue;
                        }

                        let path = dir_entry.path();
                        let stem = match path.file_stem() {
                            Some(stem) => stem,
                            None => {
                                log::error!("Can't extract file stem from '{}'.", dir_entry.path().display());
                                continue;
                            }
                        };

                        if stem == CACHE_ENTRY_DATA_FILENAME {
                            // found
                            data_path = Some(path.to_owned());
                            break;
                        }
                    },
                    Err(e) => log::error!("Can't read directory entry:\n{}", e)
                }
            }

            match data_path {
                Some(path) => {
                    // use this data file at path
                },
                None => log::error!("Data file not found at entry '{}'", entry.borrow().location.display())
            }
            */
        }

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

        log::info!("Generating atlas data at {}...", output_atlas_data_path.display());
        if self.prettify_output {
            atlas_data.save_pretty_to_path(output_atlas_data_path).unwrap();
        } else {
            atlas_data.save_to_path(output_atlas_data_path).unwrap();
        }
        log::info!("Done!")
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
