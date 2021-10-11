use std::{
    fs, io,
    path::{Path, PathBuf},
};

use colored::Colorize;
use directories::ProjectDirs;

use tree_decorator::{close_tree_item, decorator};

use crate::{
    common::Verbosity,
    modes::generator::processors::{
        cache::CacheMetadata, data::FrameData, ConfigStatus, Processor, State,
    },
    settings::{Config, ProcessorConfig},
    util::{self, Timer},
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

        let cache_pathbuf =
            PathBuf::from(&state.config.cache.path).join(&state.config.cache.identifier);

        if cache_pathbuf.is_dir() {
            traceln!("Cleaning directories");

            // remove directory
            fs::remove_dir_all(&cache_pathbuf).unwrap();
            util::wait_until(|| !cache_pathbuf.is_dir());
        }

        // create cache instance root directory
        fs::create_dir_all(&cache_pathbuf).unwrap();
        util::wait_until(|| cache_pathbuf.is_dir());

        // atlas subdir
        traceln!("Creating {} sub directory", "atlas".bold());
        self.ensure_exists_subdir(&cache_pathbuf, "atlas").unwrap();

        // images subdir
        traceln!("Creating {} sub directory", "images".bold());
        self.ensure_exists_subdir(&cache_pathbuf, "images").unwrap();

        traceln!("Initializing cache file");
        let mut cache = Cache::new(
            metadata,
            state.config.cache.images_path(),
            state.config.cache.atlas_path(),
        );

        // always start outdated
        cache.mark_as_outdated();

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

        infoln!(block, "Verifying");
        self.verify_cache_status(&state.config.image.input_path, cache);
        doneln!();

        // remove invalid cache entries
        // checking if directory entry still exists
        infoln!(block, "Removing invalid cache entries");
        let cache_images_path = state.config.cache.images_path();
        let removing_entries_timer = Timer::start();

        cache.files.retain(|location, entry| {
            let cache_entry_path = cache_images_path.join(location);

            let retain = if entry.borrow().is_invalid() {
                traceln!("'{}': Source is invalid", cache_entry_path.display());

                false
            } else {
                match cache_entry_path.metadata() {
                    Ok(metadata) => {
                        if !metadata.is_dir() {
                            // don't keep it
                            traceln!("'{}': Isn't a valid directory", cache_entry_path.display());

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
            };

            if !retain && cache_entry_path.exists() {
                // delete cache directory entry
                fs::remove_dir_all(cache_entry_path).unwrap();
            }

            retain
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
            Err(io_error) => match &io_error.kind() {
                io::ErrorKind::NotFound => {
                    traceln!(
                        "Cache output directory path '{}' doesn't seems to exist",
                        cache_dir_pathbuf.display()
                    );
                    traceln!(entry: decorator::Entry::None, "It'll be created right now");

                    fs::create_dir_all(&cache_dir_pathbuf).unwrap();
                    util::wait_until(|| cache_dir_pathbuf.exists());

                    infoln!(last, "Cache output directory created!");
                }
                _ => {
                    panic!(
                        "When trying to access directory '{}' metadata: {}",
                        cache_dir_pathbuf.display(),
                        io_error
                    );
                }
            },
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
            "At source root path {}",
            source_root_path.display().to_string().bold()
        );

        let mut invalid_entries = 0;
        let mut missing_registered_cache_entries = 0;
        let mut source_modified_entries = 0;

        for (location, entry_ref) in cache.files.iter_mut() {
            let mut is_invalid_entry = false;
            let mut is_modified_entry = false;
            let mut is_missing_registered_cache_entry = false;

            let pathbuf = source_root_path
                .join(location)
                .with_extension(&entry_ref.borrow().extension);

            match pathbuf.metadata() {
                Ok(metadata) => {
                    if !metadata.is_file() {
                        // source file doesn't exists anymore
                        traceln!("[{}]: Source isn't a file", location.display());
                        is_invalid_entry = true;
                    }

                    if metadata.modified().unwrap() != entry_ref.borrow().modtime {
                        // source file modtime doesn't matches cache entry data
                        traceln!("[{}]: Source was modified", location.display());
                        is_modified_entry = true;
                    }
                }
                Err(e) => {
                    if let io::ErrorKind::NotFound = e.kind() {
                        // file wasn't found
                        traceln!("[{}]: Source was not found", location.display());
                        is_invalid_entry = true;
                    } else {
                        panic!("{}", e);
                    }
                }
            }

            // verify if cache entry's directory and it's expected contents exists
            let cache_entry_dir_path = cache.images_path.join(location);

            if !cache_entry_dir_path.is_dir() {
                is_missing_registered_cache_entry = true;
            } else {
                let entry = entry_ref.borrow();

                // check the existence of every frame file
                for (index, frame) in entry.data.frames.iter().enumerate() {
                    match frame {
                        // ignoring completely blank frame
                        // file may not exist because of that
                        FrameData::Empty => (),

                        FrameData::Contents { .. } => {
                            let frame_filepath =
                                cache_entry_dir_path.join(format!("{}.png", index));

                            match frame_filepath.metadata() {
                                Ok(m) => {
                                    if !m.is_file() {
                                        is_missing_registered_cache_entry = true;
                                        break;
                                    }
                                }
                                Err(e) => {
                                    if let io::ErrorKind::NotFound = e.kind() {
                                        is_missing_registered_cache_entry = true;
                                        break;
                                    }

                                    panic!(
                                        "Can't access expected frame file '{}': {}",
                                        frame_filepath.display(),
                                        e,
                                    );
                                }
                            }
                        }
                    }
                }

                if is_missing_registered_cache_entry {
                    // remove cache entry directory
                    fs::remove_dir_all(cache_entry_dir_path).unwrap();
                }
            }

            if is_invalid_entry {
                invalid_entries += 1;
                entry_ref.borrow_mut().mark_as_invalid();
            }

            if is_modified_entry {
                source_modified_entries += 1;
            }

            if is_missing_registered_cache_entry {
                missing_registered_cache_entries += 1;
            }
        }

        infoln!(
            "{}  {} invalid source; {} modified source; {} invalid cache entry;",
            "Entries".bold(),
            invalid_entries,
            source_modified_entries,
            missing_registered_cache_entries,
        );

        if invalid_entries > 0
            || source_modified_entries > 0
            || missing_registered_cache_entries > 0
        {
            cache.mark_as_outdated()
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
        util::wait_until(|| pathbuf.is_dir());

        Ok(pathbuf)
    }

    fn should_keep_cache(&self, previous: &CacheMetadata, current: &CacheMetadata) -> bool {
        if previous.version() != current.version() {
            return false;
        }

        true
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

    fn setup(&mut self, state: &mut State) -> ConfigStatus {
        let mut config_status = ConfigStatus::NotModified;

        infoln!(block, "Validating cache base directory");

        // handle cache output directory path
        let cache_dir_pathbuf = self.get_or_create_output(&state.config.cache.path);

        if cache_dir_pathbuf != PathBuf::from(&state.config.cache.path) {
            config_status = ConfigStatus::Modified;
            state.config.cache.path = cache_dir_pathbuf.display().to_string();
        }

        // cache identifier
        infoln!(block, "Verifying cache identifier");
        traceln!(
            entry: decorator::Entry::None,
            "At cache directory {}",
            state.config.cache.path.bold()
        );

        let identifier =
            self.get_or_create_cache_identifier(&state.config.cache.identifier, &cache_dir_pathbuf);

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

        if identifier != state.config.cache.identifier {
            config_status = ConfigStatus::Modified;
            state.config.cache.identifier = identifier;
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

        let current_metadata = state.config.cache_metadata();
        let state_cache;

        if state.args().global.force {
            infoln!(block, "Creating a new one (forced)");
            state_cache = self.initialize_cache(state, current_metadata);
            infoln!(last, "{}", "Done".green());
        } else {
            let images_path = state.config.cache.images_path();
            let atlas_output_path = state.config.cache.atlas_path();

            match Cache::load_from_path(&cache_file_pathbuf, images_path, atlas_output_path) {
                Ok(mut c) => {
                    if self.should_keep_cache(&c.meta, &current_metadata) {
                        // metadata matched expected values

                        infoln!("Version {} matches", c.meta.version().bold());
                        self.handle_cache(state, &mut c);
                        state_cache = c;
                    } else {
                        // metadata mismatch expected values

                        if current_metadata.version() != c.meta.version() {
                            infoln!("Cache is at different version");
                            infoln!(
                                entry: decorator::Entry::None,
                                "Previous version is {}",
                                c.meta.version().bold()
                            );
                        }

                        if current_metadata.generation_metadata().data.prettified
                            != c.meta.generation_metadata().data.prettified
                        {
                            infoln!(block, "Cache presenting settings mismatch");

                            if is_trace_enabled!() {
                                traceln!(
                                    last,
                                    "Data Prettified  Value: {}; Expecting: {}",
                                    c.meta.generation_metadata().data.prettified,
                                    current_metadata.generation_metadata().data.prettified
                                );
                            } else {
                                close_tree_item!();
                            }
                        }

                        state_cache = self.initialize_cache(state, current_metadata);
                    }
                }
                Err(e) => {
                    match &e {
                        super::LoadError::FileNotFound(_path) => {
                            warnln!("Cache file not found at expected path");
                            infoln!(block, "Creating a new one");
                        }
                        super::LoadError::Deserialize(serde_json_error) => {
                            match serde_json_error.classify() {
                                serde_json::error::Category::Io
                                | serde_json::error::Category::Eof => {
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

                    state_cache = self.initialize_cache(state, current_metadata);
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
