use std::{ffi::OsString, path::PathBuf};

mod frame;
mod track;

pub use frame::Frame;
pub use track::Track;

use crate::graphics::Error;

#[derive(Debug)]
pub struct Animation {
    /// File name without extension.
    pub source_name: OsString,

    /// Source file path which yields this.
    pub source_path: PathBuf,

    pub indices: Vec<u32>,
    pub frames: Vec<Frame>,
    pub tracks: Vec<Track>,
}

impl Animation {
    pub fn new(source_path: PathBuf) -> Result<Self, Error> {
        let metadata = source_path.metadata().unwrap();

        if !metadata.is_file() {
            return Err(Error::FileExpected(source_path));
        }

        let source_name = source_path
            .file_stem()
            .ok_or_else(|| Error::FileExpected(source_path.clone()))?;

        Ok(Self {
            source_name: source_name.to_owned(),
            source_path,
            indices: Vec::new(),
            frames: Vec::new(),
            tracks: Vec::new(),
        })
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub fn push_track(&mut self, track: Track) {
        self.tracks.push(track);
    }
}
