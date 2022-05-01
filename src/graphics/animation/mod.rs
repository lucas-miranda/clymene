use std::{ffi::OsString, path::PathBuf};

mod frame;
mod frame_indices;
mod frame_indices_group;
mod track;
mod track_list;

pub use frame::Frame;
pub use frame_indices::FrameIndices;
pub use frame_indices_group::FrameIndicesGroup;
pub use track::Track;
pub use track_list::TrackList;

use crate::graphics::Error;

#[derive(Debug)]
pub struct Animation {
    /// File name without extension.
    pub source_name: OsString,

    /// Source file path which yields this.
    pub source_path: PathBuf,

    pub indices: Vec<u32>,
    pub frames: Vec<Frame>,
    pub tracks: TrackList,
}

impl Animation {
    pub fn new(source_path: PathBuf) -> eyre::Result<Self> {
        let metadata = source_path.metadata().map_err(eyre::Report::from)?;

        if !metadata.is_file() {
            return Err(Error::FileExpected(source_path).into());
        }

        let source_name = source_path
            .file_stem()
            .ok_or_else::<eyre::Report, _>(|| Error::FileExpected(source_path.clone()).into())?;

        Ok(Self {
            source_name: source_name.to_owned(),
            source_path,
            indices: Vec::new(),
            frames: Vec::new(),
            tracks: TrackList::new(),
        })
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }
}
