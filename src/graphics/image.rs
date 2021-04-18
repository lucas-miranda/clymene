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
    /// Region at atlas.
    ///
    /// Used at [`PackerProcessor`], shouldn't be defined manually.
    pub atlas_region: Rectangle<u32>,

    /// File name without extension.
    pub name: OsString,

    /// Graphical file absolute path which generated this [`Image`].
    /// It's always a path to a file at intermediate folder (cache).
    pub location: PathBuf,

    /// Original file dimensions.
    pub dimensions: Size<u32>,

    /// Region, at original file, with trimmed empty spaces.
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
