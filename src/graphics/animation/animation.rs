use std::{
    collections::HashMap,
    ffi::OsString,
    path::PathBuf
};

use crate::graphics::{
    animation::Track,
    Error,
    Graphic,
    Image
};

#[derive(Debug)]
pub struct Animation {
    pub name: OsString,
    pub source_path: PathBuf,
    pub indices: Vec<u32>,
    pub source_images: HashMap<u32, Image>,
    pub tracks: Vec<Track>
}

impl Animation {
    pub fn new(source_path: PathBuf) -> Result<Self, Error> {
        let metadata = source_path.metadata().unwrap();

        if !metadata.is_file() {
            return Err(Error::FileExpected(source_path));
        }

        let name = source_path.file_stem()
                              .ok_or_else(|| Error::FileExpected(source_path.clone()))?;

        Ok(Self {
            name: name.to_owned(),
            source_path,
            indices: Vec::new(),
            source_images: HashMap::new(),
            tracks: Vec::new()
        })
    }

    pub fn insert_source_image(&mut self, index: u32, image: Image) {
        self.source_images.insert(index, image);
    }

    pub fn push_track(&mut self, track: Track) {
        self.tracks.push(track);
    }
}

impl Into<Graphic> for Animation {
    fn into(self) -> Graphic {
        Graphic::Animation(self)
    }
}
