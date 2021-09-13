use std::{
    cmp::Ordering,
    fs, io,
    path::{Path, PathBuf},
};

use colored::Colorize;

use crate::{
    common::Verbosity,
    graphics::{animation::Track, Graphic},
    math::Rectangle,
    modes::generator::processors::{
        data::{FrameData, FrameIndicesData, GraphicData, TrackData},
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
        let cache = if let Some(c) = &mut state.cache {
            c
        } else {
            infoln!("Initializing cache");
            state.cache = Some(Cache::new(
                state.config.cache.images_path(),
                state.config.cache.atlas_path(),
            ));
            state.cache.as_mut().unwrap()
        };

        // insert graphics to cache (if isn't already registered)
        let cache_images_path = state.config.cache.images_path();

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
                        .strip_prefix(&state.config.image.input_path)
                        .unwrap()
                        .with_extension("");

                    source_metadata = image.source_path.metadata().unwrap();

                    // extract data
                    data.frames.push(FrameData {
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
                        .strip_prefix(&state.config.image.input_path)
                        .unwrap()
                        .with_extension("");

                    source_metadata = source_path.metadata().unwrap();

                    // extract data
                    for track in &animation.tracks {
                        data.tracks.push(TrackData {
                            label: track.label.clone(),
                            indices: self.prepare_indices(track),
                        });
                    }

                    for (index, frame) in animation.frames.iter().enumerate() {
                        data.frames.push(
                            FrameData {
                                atlas_region: match &frame.graphic_source.atlas_region {
                                    Some(atlas_region) => atlas_region.clone(),
                                    None => {
                                        if !frame.graphic_source.region.is_empty() {
                                            errorln!("Atlas region isn't defined at Frame '{}' (graphic region: {}) from Animation '{}'", index, frame.graphic_source.region, source_path.display());
                                        }

                                        Rectangle::default()
                                    },
                                },
                                duration: Some(frame.duration),
                                source_region: frame.graphic_source.region.clone()
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

        cache.save_to_path(&cache_path).unwrap();

        Ok(())
    }

    fn prepare_indices(&self, track: &Track) -> Vec<FrameIndicesData> {
        // try to group ranges together
        let mut indices = Vec::new();
        let mut index_range_start: Option<&u32> = None;
        let mut index_range_end: Option<&u32> = None;

        for index in &track.frame_indices {
            match index_range_end {
                Some(end_index) => {
                    if *index != end_index + 1 {
                        let from = *index_range_start.expect("Undefined range start.");
                        let to = *index_range_end.expect("Undefined range end.");

                        match to.cmp(&from) {
                            Ordering::Greater => indices.push(FrameIndicesData::Range { from, to }),
                            Ordering::Equal => indices.push(FrameIndicesData::Value(to)),
                            _ => panic!("Malformed indices array. From: {}, To: {}", from, to),
                        }

                        index_range_start = Some(index);
                    }
                }
                None => {
                    index_range_start = Some(index);
                }
            }

            index_range_end = Some(index);
        }

        // handle remaining indices
        if let Some(from) = index_range_start {
            let from = *from;
            let to = *index_range_end.expect("Undefined range end.");

            match to.cmp(&from) {
                Ordering::Greater => indices.push(FrameIndicesData::Range { from, to }),
                Ordering::Equal => indices.push(FrameIndicesData::Value(to)),
                _ => panic!("Malformed indices array. From: {}, To: {}", from, to),
            }
        }

        indices
    }
}

impl Processor for CacheExporterProcessor {
    fn name(&self) -> &str {
        "Cache Exporter"
    }

    fn retrieve_processor_config<'a>(&self, config: &'a Config) -> &'a dyn ProcessorConfig {
        &config.cache
    }

    fn setup(&mut self, _config: &mut Config) -> ConfigStatus {
        ConfigStatus::NotModified
    }

    fn execute(&self, state: &mut State) {
        infoln!(block, "Cache result");
        let total_timer = Timer::start();
        infoln!("Preparing to update entries");

        let cache_dir_pathbuf = state.config.cache.root_path();
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
