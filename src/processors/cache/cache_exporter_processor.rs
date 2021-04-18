use std::{
    fs,
    io,
    path::PathBuf
};

use crate::{
    graphics::Graphic,
    processors::{
        cache::{
            self,
            Cache,
        },
        ConfigStatus,
        Data,
        Error,
        Processor
    },
    settings::Config
};

pub struct CacheExporterProcessor {
}

impl Processor for CacheExporterProcessor {
    fn setup(&mut self, _config: &mut Config) -> Result<ConfigStatus, Error> {
        Ok(ConfigStatus::NotModified)
    }

    fn execute(&self, data: &mut Data) -> Result<(), Error> {
        let cache_dir_pathbuf = data.config.cache.root_path();
        let cache_pathbuf = cache_dir_pathbuf.join(Cache::default_filename());

        if let Err(e) = fs::remove_file(&cache_pathbuf) {
            match e.kind() {
                io::ErrorKind::NotFound => (),
                _ => return Err(Error::CacheProcessor(cache::Error::IO(e)))
            }
        }

        let cache = if let Some(c) = &mut data.cache {
            c
        } else {
            data.cache = Some(Cache::new());
            data.cache.as_mut().unwrap()
        };

        // insert graphics to cache (if isn't already registered)
        let cache_images_path = data.config.cache.images_path();
        for g in data.graphic_output.graphics.iter() {
            let graphic_path = match g {
                Graphic::Image(image) => {
                    match image.location.parent() {
                        Some(parent_path) => parent_path,
                        None => panic!("Parent path not found at image location '{}'.", image.location.display())
                    }
                },
                Graphic::Animation(animation) => {
                    &animation.directory_location
                },
                Graphic::Empty => continue
            };

            let location = match graphic_path.strip_prefix(&cache_images_path) {
                Ok(path) => path,
                Err(e) => {
                    panic!("Trying to strip root path '{}' from graphic's path '{}': {}", cache_images_path.display(), graphic_path.display(), e);
                }
            };

            // verify if directory really exists
            // and cache it, if positive
            let cache_graphic_entry_path = cache_images_path.join(location);
            match cache_graphic_entry_path.metadata() {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        cache.register(location, &metadata)
                             .map_err(Error::CacheProcessor)?;

                        log::trace!("Cache path '{}'", cache_graphic_entry_path.display());
                    } else {
                        log::error!("Ignoring, expected a directory at graphic's cache path '{}'.", cache_graphic_entry_path.display());
                    }
                },
                Err(e) => {
                    panic!("Trying to verify graphic's cache directory path '{}' metadata: {}", cache_graphic_entry_path.display(), e);
                }
            }
        }

        // write cache to file
        cache.save_to_path(&cache_pathbuf)
             .map_err(|e| Error::CacheProcessor(cache::Error::Save(e)))?;

        Ok(())
    }
}

impl CacheExporterProcessor {
    pub fn new() -> Self {
        Self {
        }
    }
}
