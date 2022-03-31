use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

use serde::{Deserialize, Serialize};

use crate::{
    graphics::{
        animation::{Animation, Frame},
        Graphic, Image,
    },
    modes::generator::processors::{
        data::{FrameData, GraphicData},
        image::{GraphicSourceData, GraphicSourceDataSet},
    },
};

#[derive(Serialize, Deserialize)]
pub struct CacheEntry {
    /// Last source file modified time>
    pub modtime: SystemTime,

    /// Source file extension
    pub extension: String,

    /// Graphic's data
    pub data: GraphicData,

    /// Quickly access to location.
    /// It's the path from cache root directory to this entry with stripped root directory.
    #[serde(skip)]
    pub location: PathBuf,

    #[serde(skip)]
    invalid: bool,
}

impl CacheEntry {
    pub fn new(
        modtime: SystemTime,
        extension: String,
        data: GraphicData,
        location: PathBuf,
    ) -> Self {
        Self {
            modtime,
            extension,
            data,
            location,
            invalid: false,
        }
    }

    pub fn retrieve_graphic(&self, source_path: &Path, image_root_path: &Path) -> Option<Graphic> {
        let graphic_dir_path = image_root_path.join(&self.location);
        let mut graphic_source_data_set = GraphicSourceDataSet::new();

        // collect files' paths
        for entry in fs::read_dir(&graphic_dir_path)
            .unwrap()
            .filter_map(|e| e.ok())
        {
            if let Ok(graphic_source_data) =
                GraphicSourceData::try_from_path(&entry.path(), &self.data.frames)
            {
                graphic_source_data_set.sources.push(graphic_source_data)
            }
        }

        graphic_source_data_set
            .sources
            .sort_unstable_by(|a, b| a.frame_index.cmp(&b.frame_index));

        // build graphic

        if graphic_source_data_set.sources.is_empty() {
            return Some(Graphic::Empty);
        }

        if graphic_source_data_set.sources.len() == 1 && self.data.tracks.is_empty() {
            // single image
            return Some(
                Image::with_graphic_source(
                    graphic_source_data_set.sources.remove(0).source,
                    source_path.to_owned(),
                )
                .unwrap()
                .into(),
            );
        }

        let mut animation = match Animation::new(source_path.to_owned()) {
            Ok(anim) => anim,
            Err(_) => return None,
        };

        // register source images
        for (frame_index, source_data) in graphic_source_data_set.sources.drain(..).enumerate() {
            match self.data.frames.get(frame_index) {
                Some(frame_data) => animation.push_frame(match frame_data {
                    FrameData::Empty => Frame::Empty,
                    FrameData::Contents { duration, .. } => Frame::Contents {
                        graphic_source: source_data.source,
                        duration: duration.unwrap_or_default(),
                    },
                }),
                None => panic!(
                    "Frame {} data not found. From cache entry at '{}'.",
                    frame_index,
                    self.location.display()
                ),
            }
        }

        // register tracks
        for track in self.data.tracks.entries() {
            animation.tracks.register(track.clone())
        }

        Some(animation.into())
    }

    pub fn mark_as_invalid(&mut self) {
        self.invalid = true;
    }

    pub fn is_invalid(&self) -> bool {
        self.invalid
    }
}
