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

    /// File location
    pub path: PathBuf,

    /// Source file path which yields this.
    pub source_path: PathBuf,

    /// Original file dimensions.
    pub dimensions: Size<u32>,

    /// Region, at original file, with trimmed empty spaces.
    pub source_region: Rectangle<u32>
}

impl Image {
    pub fn new(path: PathBuf, source_path: PathBuf, dimensions: Size<u32>, source_region: Rectangle<u32>) -> Result<Self, Error> {
        let metadata = source_path.metadata().unwrap();

        if !metadata.is_file() {
            return Err(Error::FileExpected(source_path.clone()));
        }

        let name = source_path.file_stem()
                              .ok_or(Error::FileExpected(source_path.clone()))?;

        Ok(Self {
            atlas_region: Rectangle::default(),
            name: name.to_owned(),
            path,
            source_path,
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
