use std::{
    fs,
    io,
    path::{
        Path,
        PathBuf
    }
};

use colored::Colorize;
use directories::ProjectDirs;

use log::{
    info,
    trace,
    warn
};

use crate::{
    common::Verbosity,
    processors::{
        cache::{
            self,
            Cache
        },
        ConfigStatus,
        Processor,
        State
    },
    settings::{
        Config,
        ProcessorConfig
    }
};

pub struct CacheImporterProcessor {
    verbose: bool
}

impl CacheImporterProcessor {
    pub fn new() -> Self {
        Self {
            verbose: false
        }
    }

    fn generate_hash() -> String {
        use rand::Rng;

        const HASH_LENGTH: usize = 10;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                 abcdefghijklmnopqrstuvwxyz\
                                 0123456789";

        let mut rng = rand::thread_rng();

        (0..HASH_LENGTH)
           .map(|_| {
               let i = rng.gen_range(0, CHARSET.len());
               CHARSET[i] as char
           })
           .collect::<String>()
    }

    fn get_default_cache_path() -> Option<PathBuf> {
        ProjectDirs::from("io", "Raven", "Raven")
                    .map(|project_dirs| PathBuf::from(project_dirs.cache_dir()))
    }

    fn create_subdir(&self, cache_pathbuf: &Path, dir_name: &str) -> Result<PathBuf, cache::Error> {
        let pathbuf = cache_pathbuf.join(dir_name);

        match pathbuf.metadata() {
            Ok(metadata) => {
                if metadata.is_dir() {
                    Ok(())
                } else {
                    Err(cache::Error::DirectoryExpected(pathbuf.clone()))
                }
            },
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => Ok(()),
                    _ => panic!("{}", e)
                }
            }
        }?;

        fs::create_dir_all(&pathbuf).unwrap();

        while !pathbuf.is_dir() {
            std::thread::sleep(std::time::Duration::from_millis(10u64));
            continue;
        }

        Ok(pathbuf)
    }
}

impl Processor for CacheImporterProcessor {
    fn name(&self) -> &str {
        "Cache Importer"
    }

    fn retrieve_processor_config<'a>(&self, config: &'a Config) -> &'a dyn ProcessorConfig {
        &config.cache
    }

    fn setup(&mut self, config: &mut Config) -> ConfigStatus {
        let mut config_status = ConfigStatus::NotModified;

        // handle cache output directory path
        let cache_dir_pathbuf = if config.cache.path.is_empty() {
            config_status = ConfigStatus::Modified;
            match Self::get_default_cache_path() {
                Some(default_cache_path) => default_cache_path,
                None => panic!("Failed to retrieve default cache path.")
            }
        } else {
            PathBuf::from(&config.cache.path)
        };

        match cache_dir_pathbuf.metadata() {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    panic!("Cache path '{}' isn't a valid directory.", cache_dir_pathbuf.display());
                }
            },
            Err(io_error) => {
                match &io_error.kind() {
                    io::ErrorKind::NotFound => {
                        trace!(
                            "{}  Cache output dir path '{}' doesn't seems to exist, it'll be created right now...", 
                            "Raven".bold(), 
                            cache_dir_pathbuf.display()
                        );

                        fs::create_dir_all(&cache_dir_pathbuf).unwrap();

                        // wait until directory is created
                        while !cache_dir_pathbuf.exists() {
                            std::thread::sleep(std::time::Duration::from_millis(10u64));
                        }

                        info!("{}  Cache output directory created!", "Raven".bold());
                    },
                    _ => {
                        panic!("When trying to access directory '{}' metadata: {}", cache_dir_pathbuf.display(), io_error);
                    }
                }
            }
        }

        config.cache.path = cache_dir_pathbuf.display().to_string();

        // cache identifier

        trace!("Verifying cache identifier...");
        let mut identifier = config.cache.identifier.clone();
        let generate_identifier;

        if identifier.is_empty() {
            info!("* Cache identifier not set.");
            generate_identifier = true;
        } else {
            match cache_dir_pathbuf.join(&identifier).metadata() {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        generate_identifier = false;
                        info!("* Cache instance found with identifier '{}'.", identifier);
                    } else {
                        generate_identifier = !metadata.is_dir();
                        warn!("* Previous cache instance with identifier '{}' can't be used.", identifier);
                    }
                },
                Err(io_error) => {
                    match &io_error.kind() {
                        io::ErrorKind::NotFound => {
                            generate_identifier = false;
                            info!("* Cache identifier '{}' is unused, it'll be used.", identifier);
                        },
                        _ => generate_identifier = true
                    }
                }
            }
        }

        if generate_identifier {
            info!("* Generating a new one...");

            let mut tries = 100;
            while tries > 0 {
                identifier = CacheImporterProcessor::generate_hash();

                match cache_dir_pathbuf.join(&identifier).metadata() {
                    Ok(_metadata) => (),
                    Err(io_error) => {
                        if let io::ErrorKind::NotFound = io_error.kind() {
                            // we can use the generated hash
                            break;
                        }
                    }
                };

                tries -= 1;
            }

            if tries <= 0 {
                panic!("Exceeded max tries. Can't generate a valid identifier.");
            }

            config_status = ConfigStatus::Modified;
            info!("* Current instance cache identifier is '{}'.", identifier);
        }

        // create cache instance path
        let cache_instance_path = cache_dir_pathbuf.join(&identifier);

        match cache_instance_path.metadata() {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    panic!("Cache instance path '{}' isn't a valid directory.", cache_instance_path.display());
                }
            },
            Err(e) => {
                if let io::ErrorKind::NotFound = e.kind() {
                    fs::create_dir(&cache_instance_path).unwrap();
                } else {
                    panic!("Failed to access cache instance directory metadata at '{}': {}", cache_instance_path.display(), e);
                }
            }
        }

        config.cache.identifier = identifier;

        config_status
    }

    fn execute(&self, state: &mut State) {
        info!("> Checking cache version...");

        let cache_dir_pathbuf = state.config.cache.root_path();
        let cache_pathbuf = cache_dir_pathbuf.join(Cache::default_filename());

        trace!("- At file '{}'", cache_pathbuf.display());
        let mut cache = match Cache::load_from_path(&cache_pathbuf) {
            Ok(c) => c,
            Err(e) => {
                warn!("  Cache file not found at expected path.");
                match &e {
                    cache::LoadError::FileNotFound(path) => {
                        info!("  Creating a new one...");
                        let c = Cache::new(state.config.cache.images_path(), state.config.cache.atlas_path());

                        c.save_to_path(&path)
                         .unwrap();

                        c
                    }
                    _ => panic!("{}", e)
                }
            }
        };

        let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");

        let cache_images_path = state.config.cache.images_path();
        match cache.meta.expect_version(&version) {
            Ok(_) => {
                info!("|- Ok!");

                // atlas subdir
                self.create_subdir(&cache_dir_pathbuf, "atlas").unwrap();

                // images subdir
                self.create_subdir(&cache_dir_pathbuf, "images").unwrap();

                // remove invalid cache's entries
                // checking if directory entry still exists
                cache.files.retain(|path, _entry| {
                    let cache_entry_path = cache_images_path.join(path);
                    match cache_entry_path.metadata() {
                        Ok(metadata) => {
                            if !metadata.is_dir() {
                                // don't keep it
                                log::trace!("Removing invalid cache entry at '{}': Isn't a valid directory.", cache_entry_path.display());
                                false
                            } else {
                                true
                            }
                        },
                        Err(e) => {
                            // don't keep it
                            match e.kind() {
                                io::ErrorKind::NotFound => {
                                    log::trace!("Removing invalid cache entry at '{}'. Path not found.", cache_entry_path.display());
                                },
                                _ => {
                                    log::error!("At file '{}', io error: {}", cache_entry_path.display(), e);
                                }
                            }

                            false
                        }
                    }
                });
            },
            Err(e) => {
                match &e {
                    cache::Error::InvalidVersion { .. } => {
                        warn!("| {}", &e);

                        let cache_pathbuf = PathBuf::from(&state.config.cache.path)
                                                    .join(&state.config.cache.identifier);

                        if cache_pathbuf.is_dir() {
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
                        self.create_subdir(&cache_pathbuf, "atlas").unwrap();

                        // images subdir
                        self.create_subdir(&cache_pathbuf, "images").unwrap();

                        info!("> Initializing cache file");
                        cache = Cache::new(state.config.cache.images_path(), state.config.cache.atlas_path());
                        cache.save_to_path(&cache_pathbuf).unwrap();

                        info!("|- Created at '{}'", cache_pathbuf.display());
                        info!("|-- Cache instance created!");
                    },
                    _ => panic!("{}", e)
                }
            }
        }

        state.cache.replace(cache);
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
