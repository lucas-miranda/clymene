use std::{
    ffi::OsString,
    path::PathBuf
};

use crate::{
    graphics::{
        Error,
        Graphic
    },
    math::{
        Rectangle,
        Size
    }
};

#[derive(Debug)]
pub struct Image {
    pub atlas_region: Rectangle<u32>,
    pub name: OsString,
    pub location: PathBuf,
    pub dimensions: Size<u32>,
    pub source_region: Rectangle<u32>
}

impl Image {
    pub fn new(location: PathBuf, dimensions: Size<u32>, source_region: Rectangle<u32>) -> Result<Self, Error> {
        let metadata = location.metadata()?;

        if !metadata.is_file() {
            return Err(Error::FileExpected);
        }

        let name = location.file_stem()
                           .ok_or(Error::FileExpected)?;

        Ok(Self {
            atlas_region: Rectangle::default(),
            name: name.to_owned(),
            location,
            dimensions,
            source_region
        })
    }
}

impl Into<Graphic> for Image {
    fn into(self) -> Graphic {
        Graphic::Image(self)
    }
}
