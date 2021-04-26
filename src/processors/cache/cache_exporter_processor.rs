use std::{
    fs,
    io,
    path::Path
};

use crate::{
    graphics::Graphic,
    processors::{
        cache::Cache,
        ConfigStatus,
        Data,
        Processor
    },
    settings::Config
};

pub struct CacheExporterProcessor {
}

impl Processor for CacheExporterProcessor {
    fn name(&self) -> &str {
        "Cache Exporter"
    }

    fn setup(&mut self, _config: &mut Config) -> ConfigStatus {
        ConfigStatus::NotModified
    }

    fn execute(&self, data: &mut Data) {
        let cache_dir_pathbuf = data.config.cache.root_path();
        let cache_pathbuf = cache_dir_pathbuf.join(Cache::default_filename());

        if let Err(e) = fs::remove_file(&cache_pathbuf) {
            match e.kind() {
                io::ErrorKind::NotFound => (),
                _ => panic!("Can't remove cache file at '{}': {}", cache_pathbuf.display(), e)
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
            let source_path;
            let location: &Path;
            let source_metadata;

            let graphic_cache_dir_path = match g {
                Graphic::Image(image) => {
                    source_path = image.source_path.with_extension("");
                    location = source_path.strip_prefix(&data.config.image.input_path).unwrap();
                    source_metadata = image.source_path.metadata().unwrap();

                    cache_images_path.join(&location)
                },
                Graphic::Animation(animation) => {
                    source_path = animation.source_path.with_extension("");
                    location = source_path.strip_prefix(&data.config.image.input_path).unwrap();
                    source_metadata = animation.source_path.metadata().unwrap();

                    cache_images_path.join(&location)
                },
                Graphic::Empty => continue
            };

            // verify if directory really exists
            // and cache it, if positive
            match graphic_cache_dir_path.metadata() {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        cache.register(location, &source_metadata)
                             .unwrap();

                        log::trace!("Cache path '{}'", graphic_cache_dir_path.display());
                    } else {
                        log::error!("Ignoring, expected a directory at graphic's cache path '{}'.", graphic_cache_dir_path.display());
                    }
                },
                Err(e) => {
                    panic!("Trying to verify graphic's cache directory path '{}' metadata: {}", graphic_cache_dir_path.display(), e);
                }
            }
        }

        // write cache to file
        cache.save_to_path(&cache_pathbuf).unwrap();
    }
}

impl CacheExporterProcessor {
    pub fn new() -> Self {
        Self {
        }
    }
}
