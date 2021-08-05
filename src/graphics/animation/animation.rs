use std::{
    ffi::OsString,
    path::{
        Path,
        PathBuf
    }
};

use crate::graphics::{
    animation::{
        Frame,
        Track,
    },
    Error,
    Graphic,
    GraphicSource
};

#[derive(Debug)]
pub struct Animation {
    /// File name without extension.
    pub source_name: OsString,

    /// Source file path which yields this.
    pub source_path: PathBuf,

    pub indices: Vec<u32>,
    pub frames: Vec<Frame>,
    pub tracks: Vec<Track>
}

impl Animation {
    pub fn new(source_path: PathBuf) -> Result<Self, Error> {
        let metadata = source_path.metadata().unwrap();

        if !metadata.is_file() {
            return Err(Error::FileExpected(source_path));
        }

        let source_name = source_path.file_stem()
                                     .ok_or_else(|| Error::FileExpected(source_path.clone()))?;

        Ok(Self {
            source_name: source_name.to_owned(),
            source_path,
            indices: Vec::new(),
            frames: Vec::new(),
            tracks: Vec::new()
        })
    }

    pub fn push_frame(&mut self, graphic_source: GraphicSource, duration: u32) {
        self.frames.push(
            Frame {
                graphic_source,
                duration
            }
        );
    }

    pub fn push_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    pub fn location(&self, source_root_directory: &Path) -> Option<PathBuf> {
        match self.source_path.strip_prefix(&source_root_directory) {
            Ok(path) => Some(path.with_extension("")),
            Err(_) => None
        }
    }
}

impl From<Animation> for Graphic {
    fn from(animation: Animation) -> Self {
        Graphic::Animation(animation)
    }
}
