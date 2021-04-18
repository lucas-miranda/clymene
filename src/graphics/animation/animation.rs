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
    pub directory_location: PathBuf,
    pub indices: Vec<u32>,
    pub source_images: HashMap<u32, Image>,
    pub tracks: Vec<Track>
}

impl Animation {
    pub fn new(directory_location: PathBuf) -> Result<Self, Error> {
        let metadata = directory_location.metadata()?;

        if !metadata.is_dir() {
            return Err(Error::DirectoryExpected);
        }

        let name = directory_location.file_name()
                                     .ok_or(Error::DirectoryExpected)?;

        Ok(Self {
            name: name.to_owned(),
            directory_location,
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
