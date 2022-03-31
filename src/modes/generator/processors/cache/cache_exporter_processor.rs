use std::{
    fs, io,
    path::{Path, PathBuf},
};

use colored::Colorize;

use crate::{
    common::Verbosity,
    graphics::{animation::Frame, Graphic},
    math::Rectangle,
    modes::generator::processors::{
        data::{FrameData, GraphicData},
        ConfigStatus, Processor, State,
    },
    settings::{Config, ProcessorConfig},
    util::Timer,
};

use super::Cache;

pub struct CacheExporterProcessor {
    verbose: bool,
}

impl CacheExporterProcessor {
    pub fn new() -> Self {
        Self { verbose: false }
    }

    fn backup(&self, cache_path: &Path) -> Result<Option<PathBuf>, io::Error> {
        if !cache_path.is_file() {
            traceln!("No need to backup, there is no cache.json file");
            return Ok(None);
        }

        let mut backup_filename = cache_path.file_name().unwrap().to_owned();
        backup_filename.push(".backup");
        let backup_cache_path = cache_path.with_file_name(backup_filename);
        fs::rename(cache_path, &backup_cache_path)?;
        traceln!(
            "Backup previous cache.json to {}",
            backup_cache_path.display().to_string().bold()
        );

        Ok(Some(backup_cache_path))
    }

    fn cache(&self, state: &mut State, cache_path: &Path) -> Result<(), super::SaveError> {
        let c = state.config.try_read().expect("Can't retrieve a read lock");
        let current_metadata = state.create_cache_metadata();
        let debug = state.args().global.debug;

        let cache = if let Some(c) = &mut state.cache {
            if c.meta != current_metadata {
                c.meta = current_metadata;
            }

            c
        } else {
            infoln!("Initializing cache");

            state.cache = Some(Cache::new(
                current_metadata,
                c.cache.images_path(),
                c.cache.atlas_path(),
            ));

            state.cache.as_mut().unwrap()
        };

        // insert graphics to cache (if isn't already registered)
        let cache_images_path = c.cache.images_path();

        infoln!(block, "Registering generated graphics and data");
        let timer = Timer::start();
        for g in state.graphic_output.graphics.iter() {
            let source_path;
            let location;
            let source_metadata;
            let mut data = GraphicData::new();

            let graphic_cache_dir_path = match g {
                Graphic::Image(image) => {
                    source_path = &image.source_path;
                    location = source_path
                        .strip_prefix(&c.image.input_path)
                        .unwrap()
                        .with_extension("");

                    source_metadata = image.source_path.metadata().unwrap();

                    // extract data
                    data.frames.push(FrameData::Contents {
                        atlas_region: match &image.graphic_source.atlas_region {
                            Some(atlas_region) => atlas_region.clone(),
                            None => panic!(
                                "Expected atlas region isn't defined at Image '{}'",
                                image.source_path.display()
                            ),
                        },
                        duration: None,
                        source_region: image.graphic_source.region.clone(),
                    });

                    cache_images_path.join(&location)
                }
                Graphic::Animation(animation) => {
                    source_path = &animation.source_path;
                    location = source_path
                        .strip_prefix(&c.image.input_path)
                        .unwrap()
                        .with_extension("");

                    source_metadata = source_path.metadata().unwrap();

                    // extract data
                    for track in animation.tracks.entries() {
                        data.tracks.register(track.clone());
                    }

                    for (index, frame) in animation.frames.iter().enumerate() {
                        data.frames.push(
                            match frame {
                                Frame::Empty => FrameData::Empty,
                                Frame::Contents { graphic_source, duration } => FrameData::Contents {
                                    atlas_region: match &graphic_source.atlas_region {
                                        Some(atlas_region) => atlas_region.clone(),
                                        None => {
                                            if !graphic_source.region.is_empty() {
                                                errorln!("Atlas region isn't defined at Frame '{}' (graphic region: {}) from Animation '{}'", index, graphic_source.region, source_path.display());
                                            }

                                            Rectangle::default()
                                        },
                                    },
                                    duration: Some(*duration),
                                    source_region: graphic_source.region.clone()
                                }
                            }
                        );
                    }

                    cache_images_path.join(&location)
                }
                Graphic::Empty => continue,
            };

            // verify if directory really exists
            // and cache it, if positive
            match graphic_cache_dir_path.metadata() {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        let ext = source_path
                            .extension()
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_or_default()
                            .to_owned();

                        cache
                            .register(location, ext, &source_metadata, data)
                            .unwrap();
                    }
                }
                Err(e) => {
                    panic!(
                        "Trying to verify graphic's cache directory path '{}' metadata: {}",
                        graphic_cache_dir_path.display(),
                        e
                    );
                }
            }
        }

        doneln_with_timer!(timer);

        // write cache to file
        infoln!("Writing to file");
        traceln!("At {}", cache_path.display().to_string().bold());

        if debug && c.prettify {
            cache.save_pretty_to_path(&cache_path).unwrap();
        } else {
            cache.save_to_path(&cache_path).unwrap();
        }

        Ok(())
    }
}

impl Processor for CacheExporterProcessor {
    fn name(&self) -> &str {
        "Cache Exporter"
    }

    fn retrieve_processor_config<'a>(&self, config: &'a Config) -> Option<&'a dyn ProcessorConfig> {
        Some(&config.cache)
    }

    fn setup(&mut self, _state: &mut State) -> ConfigStatus {
        ConfigStatus::NotModified
    }

    fn execute(&mut self, state: &mut State) {
        infoln!(block, "Cache result");
        let total_timer = Timer::start();
        infoln!("Preparing to update entries");

        let cache_dir_pathbuf = {
            let c = state.config.try_read().expect("Can't retrieve a read lock");
            c.cache.root_path()
        };

        let cache_pathbuf = cache_dir_pathbuf.join(Cache::default_filename());

        let backup_path = match self.backup(cache_pathbuf.as_path()) {
            Ok(p) => p,
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => (),
                    _ => panic!(
                        "Can't backup cache file at '{}': {}",
                        cache_pathbuf.display(),
                        e
                    ),
                }

                None
            }
        };

        match self.cache(state, cache_pathbuf.as_path()) {
            Ok(()) => {
                if let Some(p) = backup_path {
                    // remove backup
                    if let Err(e) = fs::remove_file(&p) {
                        match e.kind() {
                            io::ErrorKind::NotFound => (),
                            _ => {
                                panic!("Can't remove backup cache file at '{}': {}", p.display(), e)
                            }
                        }
                    } else {
                        traceln!(
                            "Removed old cache file at {}",
                            p.display().to_string().bold()
                        );
                    }
                }
            }
            Err(e) => {
                if let Some(p) = backup_path {
                    // return previous version

                    // remove failed cache file
                    if let Err(e) = fs::remove_file(&cache_pathbuf) {
                        match e.kind() {
                            io::ErrorKind::NotFound => (),
                            _ => {
                                panic!("Can't remove backup cache file at '{}': {}", p.display(), e)
                            }
                        }
                    } else {
                        traceln!(
                            "Removing try to cache file at {}",
                            p.display().to_string().bold()
                        );
                        traceln!("Because of error: {}", e);
                    }

                    // return backup to previous state
                    let original_filename = p
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .strip_suffix(".backup")
                        .unwrap();

                    let original_path = p.with_file_name(original_filename);

                    fs::rename(p, original_path).expect("Failed to rename backup files");
                }
            }
        }

        doneln_with_timer!(total_timer);
    }
}

impl Verbosity for CacheExporterProcessor {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }
}
