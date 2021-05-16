use std::{
    cmp::Ordering,
    fs,
    io,
    path::Path
};

use crate::{
    graphics::Graphic,
    processors::{
        cache::Cache,
        ConfigStatus,
        data::{
            GraphicData,
            FrameData,
            FrameIndicesData,
            TrackData
        },
        Processor,
        State
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

    fn execute(&self, state: &mut State) {
        let cache_dir_pathbuf = state.config.cache.root_path();
        let cache_pathbuf = cache_dir_pathbuf.join(Cache::default_filename());

        if let Err(e) = fs::remove_file(&cache_pathbuf) {
            match e.kind() {
                io::ErrorKind::NotFound => (),
                _ => panic!("Can't remove cache file at '{}': {}", cache_pathbuf.display(), e)
            }
        }

        let cache = if let Some(c) = &mut state.cache {
            c
        } else {
            state.cache = Some(Cache::new(state.config.cache.images_path(), state.config.cache.atlas_path()));
            state.cache.as_mut().unwrap()
        };

        // insert graphics to cache (if isn't already registered)
        let cache_images_path = state.config.cache.images_path();
        for g in state.graphic_output.graphics.iter() {
            let source_path;
            let location: &Path;
            let source_metadata;
            let mut data = GraphicData::new();

            let graphic_cache_dir_path = match g {
                Graphic::Image(image) => {
                    source_path = image.source_path.with_extension("");
                    location = source_path.strip_prefix(&state.config.image.input_path).unwrap();
                    source_metadata = image.source_path.metadata().unwrap();

                    // extract data
                    data.frames.push(FrameData {
                        atlas_region: match &image.graphic_source.atlas_region {
                            Some(atlas_region) => atlas_region.clone(),
                            None => panic!("Expected atlas region isn't defined at Image '{}'", image.source_path.display())
                        },
                        duration: None,
                        source_region: image.graphic_source.region.clone()
                    });

                    cache_images_path.join(&location)
                },
                Graphic::Animation(animation) => {
                    source_path = animation.source_path.with_extension("");
                    location = source_path.strip_prefix(&state.config.image.input_path).unwrap();
                    source_metadata = animation.source_path.metadata().unwrap();

                    // extract data
                    for track in &animation.tracks {
                        // prepare indices
                        // try to group ranges together
                        let mut indices = Vec::new();
                        let mut index_range_start: Option<&u32> = None;
                        let mut index_range_end: Option<&u32> = None;

                        for index in &track.frame_indices {
                            match index_range_end {
                                Some(end_index) => {
                                    if *index == end_index + 1 {
                                        index_range_end = Some(index);
                                    } else {
                                        let from = *index_range_start.expect("Undefined range start.");
                                        let to = *index_range_end.expect("Undefined range end.");

                                        match to.cmp(&from) {
                                            Ordering::Greater => indices.push(FrameIndicesData::Range { from, to }),
                                            Ordering::Equal => indices.push(FrameIndicesData::Value(to)),
                                            _ => panic!("Malformed indices array. From: {}, To: {}", from, to)
                                        }

                                        index_range_start = Some(index);
                                        index_range_end = Some(index);
                                    }
                                },
                                None => {
                                    index_range_start = Some(index);
                                    index_range_end = Some(index);
                                }
                            }
                        }

                        // handle remaining indices
                        if let Some(from) = index_range_start {
                            let from = *from;
                            let to = *index_range_end.expect("Undefined range end.");

                            match to.cmp(&from) {
                                Ordering::Greater => indices.push(FrameIndicesData::Range { from, to }),
                                Ordering::Equal => indices.push(FrameIndicesData::Value(to)),
                                _ => panic!("Malformed indices array. From: {}, To: {}", from, to)
                            }
                        }

                        data.tracks.push(
                            TrackData {
                                label: track.label.clone(),
                                indices 
                            }
                        );
                    }

                    for frame in &animation.frames {
                        data.frames.push(FrameData {
                            atlas_region: match &frame.graphic_source.atlas_region {
                                Some(atlas_region) => atlas_region.clone(),
                                None => panic!("Expected atlas region isn't defined at Frame '{}' from Animation '{}'", animation.source_path.display(), frame.graphic_source.path.display())
                                                       
                            },
                            duration: Some(frame.duration),
                            source_region: frame.graphic_source.region.clone()
                        });
                    }

                    cache_images_path.join(&location)
                },
                Graphic::Empty => continue
            };

            // verify if directory really exists
            // and cache it, if positive
            match graphic_cache_dir_path.metadata() {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        cache.register(location, &source_metadata, data)
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
