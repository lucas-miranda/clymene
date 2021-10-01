use std::{fs, io, path::{Path, PathBuf}, thread::current};

use colored::Colorize;
use directories::ProjectDirs;

use tree_decorator::{close_tree_item, decorator};

use crate::{
    common::Verbosity,
    modes::generator::processors::{cache::CacheMetadata, ConfigStatus, Processor, State},
    settings::{CacheConfig, Config, ProcessorConfig},
    util::Timer,
};

use super::Cache;

pub struct CacheImporterProcessor {
    verbose: bool,
}

impl CacheImporterProcessor {
    pub fn new() -> Self {
        Self { verbose: false }
    }

    fn initialize_cache(&self, state: &State, metadata: CacheMetadata) -> Cache {
        traceln!(block, "Creating cache directories");

        let cache_pathbuf = PathBuf::from(&state.config.cache.path)
            .join(&state.config.cache.identifier);

        if cache_pathbuf.is_dir() {
            traceln!("Cleaning directories");

            // remove directory
            fs::remove_dir_all(&cache_pathbuf).unwrap();

            // wait until is complete removed
            while cache_pathbuf.is_dir() {
                std::thread::sleep(std::time::Duration::from_millis(10u64));
                continue;
            }
        }

        // create cache instance root directory
        fs::create_dir_all(&cache_pathbuf).unwrap();

        while !cache_pathbuf.is_dir() {
            std::thread::sleep(std::time::Duration::from_millis(10u64));
            continue;
        }

        // atlas subdir
        traceln!("Creating {} sub directory", "atlas".bold());
        self.ensure_exists_subdir(&cache_pathbuf, "atlas").unwrap();

        // images subdir
        traceln!("Creating {} sub directory", "images".bold());
        self.ensure_exists_subdir(&cache_pathbuf, "images").unwrap();

        traceln!("Initializing cache file");
        let cache = Cache::new(
            metadata,
            state.config.cache.images_path(),
            state.config.cache.atlas_path(),
        );

        let cache_file_pathbuf = cache_pathbuf.join(Cache::default_filename());
        cache.save_to_path(&cache_file_pathbuf).unwrap();

        traceln!(last, "{}", "Done".green());
        infoln!(
            entry: decorator::Entry::Double,
            "New cache instance created!"
        );

        cache
    }

    fn handle_cache(&self, state: &State, cache: &mut Cache) {
        let cache_dir_pathbuf = state.config.cache.root_path();

        // atlas subdir
        self.ensure_exists_subdir(&cache_dir_pathbuf, "atlas")
            .unwrap();

        // images subdir
        self.ensure_exists_subdir(&cache_dir_pathbuf, "images")
            .unwrap();

        self.verify_cache_status(&state.config.image.input_path, cache);

        // remove invalid cache entries
        // checking if directory entry still exists
        infoln!(block, "Removing invalid cache entries");
        let removing_entries_timer = Timer::start();
        let cache_images_path = state.config.cache.images_path();

        cache.files.retain(|path, _entry| {
            let cache_entry_path = cache_images_path.join(path);
            match cache_entry_path.metadata() {
                Ok(metadata) => {
                    if !metadata.is_dir() {
                        // don't keep it
                        traceln!(
                            "'{}': Isn't a valid directory",
                            cache_entry_path.display()
                        );
                        false
                    } else {
                        true
                    }
                }
                Err(e) => {
                    // don't keep it
                    match e.kind() {
                        io::ErrorKind::NotFound => {
                            traceln!("'{}': Path not found", cache_entry_path.display());
                        }
                        _ => {
                            errorln!(
                                "At file '{}', io error: {}",
                                cache_entry_path.display(),
                                e
                            );
                        }
                    }

                    false
                }
            }
        });

        doneln_with_timer!(removing_entries_timer);
    }

    fn get_or_create_output(&self, config_cache_path: &str) -> PathBuf {
        let cache_dir_pathbuf = if config_cache_path.is_empty() {
            match self.get_default_cache_path() {
                Some(default_cache_path) => default_cache_path,
                None => panic!("Failed to retrieve default cache path."),
            }
        } else {
            PathBuf::from(&config_cache_path)
        };

        match cache_dir_pathbuf.metadata() {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    panic!(
                        "Cache path '{}' isn't a valid directory.",
                        cache_dir_pathbuf.display()
                    );
                } else {
                    close_tree_item!();
                }
            }
            Err(io_error) => {
                match &io_error.kind() {
                    io::ErrorKind::NotFound => {
                        traceln!(
                            "Cache output directory path '{}' doesn't seems to exist",
                            cache_dir_pathbuf.display()
                        );
                        traceln!(entry: decorator::Entry::None, "It'll be created right now");

                        fs::create_dir_all(&cache_dir_pathbuf).unwrap();

                        // wait until directory is created
                        while !cache_dir_pathbuf.exists() {
                            std::thread::sleep(std::time::Duration::from_millis(10u64));
                        }

                        infoln!(last, "Cache output directory created!");
                    }
                    _ => {
                        panic!(
                            "When trying to access directory '{}' metadata: {}",
                            cache_dir_pathbuf.display(),
                            io_error
                        );
                    }
                }
            }
        }

        cache_dir_pathbuf
    }

    fn get_or_create_cache_identifier(&self, config_identifier: &str, cache_dir: &Path) -> String {
        let generate_identifier;

        if config_identifier.is_empty() {
            infoln!("Cache identifier not set");
            generate_identifier = true;
        } else {
            match cache_dir.join(&config_identifier).metadata() {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        generate_identifier = false;
                        infoln!(
                            entry: decorator::Entry::Double,
                            "Cache instance found with defined identifier {}",
                            config_identifier.bold()
                        );
                    } else {
                        generate_identifier = true;
                        traceln!(entry: decorator::Entry::None, "Directory not found");
                        warnln!(
                            "Previous cache instance with identifier {} can't be used",
                            config_identifier.bold()
                        );
                    }
                }
                Err(io_error) => match &io_error.kind() {
                    io::ErrorKind::NotFound => {
                        generate_identifier = false;
                        infoln!(
                            entry: decorator::Entry::Double,
                            "Cache identifier {} is available, it'll be used",
                            config_identifier.bold()
                        );
                    }
                    _ => generate_identifier = true,
                },
            }
        }

        if generate_identifier {
            infoln!(block, "Generating a new one");

            let mut identifier = self.generate_hash();
            let mut tries = 100;

            while tries > 0 {
                match cache_dir.join(&identifier).metadata() {
                    Ok(_metadata) => (),
                    Err(io_error) => {
                        if let io::ErrorKind::NotFound = io_error.kind() {
                            // we can use the generated hash
                            break;
                        }
                    }
                };

                identifier = self.generate_hash();
                tries -= 1;
            }

            if tries <= 0 {
                panic!("Exceeded max tries. Can't generate a valid identifier.");
            }

            infoln!(
                last,
                "Current instance cache identifier is {}",
                identifier.bold()
            );

            return identifier;
        }

        config_identifier.to_owned()
    }

    fn verify_cache_status<T: AsRef<Path>>(&self, source_root_path: T, cache: &mut Cache) {
        let source_root_path = source_root_path.as_ref();
        traceln!(
            "Verifying cache at source root path {}",
            source_root_path.display().to_string().bold()
        );

        for (location, entry_ref) in cache.files.iter() {
            let pathbuf = source_root_path
                .join(location)
                .with_extension(&entry_ref.borrow().extension);

            match pathbuf.metadata() {
                Ok(metadata) => {
                    if !metadata.is_file() {
                        // source file doesn't exists anymore
                        traceln!("[{}]: Source isn't a file", location.display());
                        cache.mark_as_outdated();
                        break;
                    }

                    let modtime = metadata.modified().unwrap();

                    if modtime != entry_ref.borrow().modtime {
                        // source file modtime doesn't matches cache entry data
                        traceln!("[{}]: Source was modified", location.display());
                        cache.mark_as_outdated();
                        break;
                    }
                }
                Err(e) => {
                    if let io::ErrorKind::NotFound = e.kind() {
                        // file wasn't found
                        traceln!("[{}]: Source was not found", location.display());
                        cache.mark_as_outdated();
                        break;
                    }

                    panic!("{}", e);
                }
            }
        }
    }

    fn ensure_exists_subdir(
        &self,
        cache_pathbuf: &Path,
        dir_name: &str,
    ) -> Result<PathBuf, super::Error> {
        let pathbuf = cache_pathbuf.join(dir_name);

        match pathbuf.metadata() {
            Ok(metadata) => {
                if metadata.is_dir() {
                    Ok(())
                } else {
                    Err(super::Error::DirectoryExpected(pathbuf.clone()))
                }
            }
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => Ok(()),
                _ => panic!("{}", e),
            },
        }?;

        fs::create_dir_all(&pathbuf).unwrap();

        while !pathbuf.is_dir() {
            std::thread::sleep(std::time::Duration::from_millis(10u64));
            continue;
        }

        Ok(pathbuf)
    }

    fn get_default_cache_path(&self) -> Option<PathBuf> {
        let crate_name = env!("CARGO_PKG_NAME");
        ProjectDirs::from("io", crate_name, crate_name)
            .map(|project_dirs| PathBuf::from(project_dirs.cache_dir()))
    }

    fn generate_hash(&self) -> String {
        use rand::Rng;

        const HASH_LENGTH: usize = 10;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                 abcdefghijklmnopqrstuvwxyz\
                                 0123456789";

        let mut rng = rand::thread_rng();

        (0..HASH_LENGTH)
            .map(|_| {
                let i = rng.gen_range(0..CHARSET.len());
                CHARSET[i] as char
            })
            .collect::<String>()
    }
}

impl Processor for CacheImporterProcessor {
    fn name(&self) -> &str {
        "Cache Importer"
    }

    fn retrieve_processor_config<'a>(&self, config: &'a Config) -> Option<&'a dyn ProcessorConfig> {
        Some(&config.cache)
    }

    fn setup(&mut self, config: &mut Config) -> ConfigStatus {
        let mut config_status = ConfigStatus::NotModified;

        infoln!(block, "Validating cache base directory");

        // handle cache output directory path
        let cache_dir_pathbuf = self.get_or_create_output(&config.cache.path);

        if cache_dir_pathbuf != PathBuf::from(&config.cache.path) {
            config_status = ConfigStatus::Modified;
            config.cache.path = cache_dir_pathbuf.display().to_string();
        }

        // cache identifier
        infoln!(block, "Verifying cache identifier");
        traceln!(
            entry: decorator::Entry::None,
            "At cache directory {}",
            config.cache.path.bold()
        );

        let identifier =
            self.get_or_create_cache_identifier(&config.cache.identifier, &cache_dir_pathbuf);

        // create cache instance path
        let cache_instance_path = cache_dir_pathbuf.join(&identifier);

        match cache_instance_path.metadata() {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    panic!(
                        "Cache instance path '{}' isn't a valid directory.",
                        cache_instance_path.display()
                    );
                } else {
                    infoln!(last, "{}", "Ok".green());
                }
            }
            Err(e) => {
                if let io::ErrorKind::NotFound = e.kind() {
                    fs::create_dir(&cache_instance_path).unwrap();
                    infoln!(last, "{}", "Ok".green());
                } else {
                    panic!(
                        "Failed to access cache instance directory metadata at '{}': {}",
                        cache_instance_path.display(),
                        e
                    );
                }
            }
        }

        if identifier != config.cache.identifier {
            config_status = ConfigStatus::Modified;
            config.cache.identifier = identifier;
        }

        config_status
    }

    fn execute(&self, state: &mut State) {
        infoln!(block, "Checking cache version");
        let total_timer = Timer::start();

        let cache_dir_pathbuf = state.config.cache.root_path();
        let cache_file_pathbuf = cache_dir_pathbuf.join(Cache::default_filename());

        traceln!(
            entry: decorator::Entry::None,
            "At file {}",
            cache_file_pathbuf.display().to_string().bold()
        );

        let current_metadata = CacheMetadata::new(state.config.data.prettify);
        let state_cache;

        if state.force {
            infoln!(block, "Creating a new one (forced)");
            state_cache = self.initialize_cache(&state, current_metadata);
            infoln!(last, "{}", "Done".green());
        } else {
            let images_path = state.config.cache.images_path();
            let atlas_output_path = state.config.cache.atlas_path();

            match Cache::load_from_path(&cache_file_pathbuf, images_path.clone(), atlas_output_path.clone()) {
                Ok(mut c) => {
                    if c.meta == current_metadata {
                        // metadata matched expected values

                        infoln!("Version {} matches", current_metadata.version.bold());
                        self.handle_cache(&state, &mut c);
                        state_cache = c;
                    } else {
                        // metadata mismatch expected values

                        if current_metadata.version != c.meta.version {
                            infoln!("Cache is at older version");
                            infoln!(
                                entry: decorator::Entry::None,
                                "Previous version is {}",
                                c.meta.version.bold()
                            );
                        }

                        if current_metadata.data_prettified != c.meta.data_prettified {
                            infoln!(block, "Cache presenting settings mismatch");

                            if is_trace_enabled!() {
                                traceln!(last, "Data Prettified  Value: {}; Expecting: {}", c.meta.data_prettified, current_metadata.data_prettified);
                            } else {
                                close_tree_item!();
                            }
                        }

                        state_cache = self.initialize_cache(&state, current_metadata);
                    }
                },
                Err(e) => {
                    match &e {
                        super::LoadError::FileNotFound(_path) => {
                            warnln!("Cache file not found at expected path");
                            infoln!(block, "Creating a new one");
                        }
                        super::LoadError::Deserialize(serde_json_error) => {
                            match serde_json_error.classify() {
                                serde_json::error::Category::Io | serde_json::error::Category::Eof => {
                                    panic!("Cache file io error: {}", e);
                                }
                                _ => {
                                    errorln!("Cache file data error: {}", serde_json_error);

                                    // TODO  maybe add a config option to panic when this situation
                                    //       happens, just to help tracing future problems

                                    infoln!("A new one will be used instead");
                                    infoln!(block, "Creating a new one");
                                }
                            }
                        }
                    }

                    state_cache = self.initialize_cache(&state, current_metadata);
                    infoln!(last, "{}", "Done".green());
                }
            }
        }

        state.cache.replace(state_cache);
        doneln_with_timer!(total_timer);
    }
}

impl Verbosity for CacheImporterProcessor {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
