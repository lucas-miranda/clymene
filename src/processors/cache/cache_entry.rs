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
    }
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
    pub fn retrieve_graphic(&self, source_path: &Path, image_root_path: &Path) -> Option<Graphic> {
        let graphic_dir_path = image_root_path.join(&self.location);
        let mut graphic_source_data_set = GraphicSourceDataSet::new();

        // collect files' paths
        for entry in fs::read_dir(&graphic_dir_path).unwrap().filter_map(|e| e.ok()) {
            if let Ok(graphic_source_data) = GraphicSourceData::try_create(&entry.path(), &self.data.frames) {
                graphic_source_data_set.sources.push(graphic_source_data)
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
                    source_path.to_owned()
                )
                .unwrap()
                .into()
            );
        }

        let mut animation = match Animation::new(source_path.to_owned()) {
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
}
