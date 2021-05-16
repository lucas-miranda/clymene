use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
    time::SystemTime
};

use serde::{ 
    Deserialize,
    Serialize
};

use crate::{
    graphics::{
        animation::{
            Animation,
            Track
        },
        Graphic,
        GraphicSource,
        Image
    },
    processors::{
        data::{
            FrameIndicesData,
            GraphicData,
        },
        image::{
            GraphicSourceData,
            GraphicSourceDataSet
        }
    },
    math::Rectangle
};

#[derive(Serialize, Deserialize)]
pub struct CacheEntry {
    /// Last source file modified time>
    pub modtime: SystemTime,

    /// Graphic's data
    pub data: GraphicData,

    #[serde(skip)]
    /// Quickly access to location.
    /// It's the path from cache root directory to this entry with stripped root directory.
    pub location: PathBuf,
}

impl CacheEntry {
    pub fn retrieve_graphic<P: AsRef<Path>>(&self, source_path: P) -> Option<Graphic> {
        let mut graphic_source_data_set = GraphicSourceDataSet::new();
        let dir_iter;

        match fs::read_dir(&self.location) {
            Ok(iter) => dir_iter = iter,
            Err(_) => return None
        };

        // collect files' paths
        for entry in dir_iter.flatten() {
            let path = entry.path();

            if let Ok(metadata) = path.metadata() {
                if !metadata.is_file() {
                    continue;
                }

                // frame index
                let frame_index: u32 = match path.file_stem() {
                    Some(stem) => {
                        match stem.to_str() {
                            Some(stem_str) => {
                                match stem_str.parse() {
                                    Ok(index) => index,
                                    Err(_) => continue
                                }
                            },
                            None => continue
                        }
                    },
                    None => continue
                };

                let source_region: Rectangle<u32>;

                match self.data.frames.get(frame_index as usize) {
                    Some(frame_data) => {
                        source_region = Rectangle::with(
                            frame_data.source_region.x,
                            frame_data.source_region.y,
                            frame_data.source_region.width,
                            frame_data.source_region.height
                        ).unwrap_or_else(Rectangle::default);
                    },
                    None => {
                        //dimensions = Size::default();
                        source_region = Rectangle::default();
                    }
                }

                graphic_source_data_set.sources.push(
                    GraphicSourceData {
                        source: GraphicSource::new(path, source_region),
                        frame_index
                    }
                );
            }
        }

        graphic_source_data_set.sources.sort_unstable_by(|a, b| a.frame_index.cmp(&b.frame_index));

        // build graphic

        if graphic_source_data_set.sources.is_empty() {
            return Some(Graphic::Empty);
        }

        if graphic_source_data_set.sources.len() == 1 && self.data.tracks.is_empty() {
            // single image
            return Some(
                Image::with_graphic_source(
                    graphic_source_data_set.sources.remove(0).source,
                    source_path.as_ref().to_owned()
                )
                .unwrap()
                .into()
            );
        }

        let mut animation = match Animation::new(self.location.to_owned()) {
            Ok(anim) => anim,
            Err(_) => return None
        };

        // register source images
        for (frame_index, source_data) in graphic_source_data_set.sources.drain(..).enumerate() {
            match self.data.frames.get(frame_index) {
                Some(frame_data) => {
                    match frame_data.duration {
                        Some(duration) => animation.push_frame(source_data.source, duration),
                        None => panic!("Animation's frame {} doesn't has a defined duration. From cache entry at '{}'.", frame_index, self.location.display())
                    }
                },
                None => panic!("Frame {} data not found. From cache entry at '{}'.", frame_index, self.location.display())
            }
        }

        // register tracks
        for track_data in &self.data.tracks {
            let label = track_data.label.as_ref().cloned();
            let mut track = Track::new(label);

            for index_entry in &track_data.indices {
                match index_entry {
                    FrameIndicesData::Value(index) => {
                        track.frame_indices.push(*index);
                    },
                    FrameIndicesData::Range { from, to } => {
                        for index in (*from)..=(*to) {
                            track.frame_indices.push(index);
                        }
                    }
                }
            }

            track.frame_indices.sort_unstable();
            animation.push_track(track);
        }

        Some(animation.into())
    }

    /*
    fn read_track_files(path: &PathBuf) -> Option<Vec<OsString>> {
        let sub_dir_iter;
        match fs::read_dir(&path) {
            Ok(iter) => sub_dir_iter = iter,
            Err(_) => return None
        };

        let mut frames = Vec::new();

        for dir_entry in sub_dir_iter {
            if let Ok(entry) = dir_entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        frames.push(entry.file_name())
                    }
                }
            }
        }

        if !frames.is_empty() {
            Some(frames)
        } else {
            None
        }
    }
    */
}
