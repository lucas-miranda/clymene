use std::path::PathBuf;

use crate::math::Rectangle;

#[derive(Debug)]
pub struct GraphicSource {
    /// Region at atlas.
    ///
    /// Defined at [`PackerProcessor`], shouldn't be changed manually.
    pub atlas_region: Option<Rectangle<u32>>,

    /// File location
    pub path: PathBuf,

    /// Exact region at original file.
    pub region: Rectangle<u32>,
}

impl GraphicSource {
    pub fn new(path: PathBuf, region: Rectangle<u32>) -> Self {
        Self {
            atlas_region: None,
            path,
            region
        }
    }
}
