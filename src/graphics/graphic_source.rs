use crate::math::Rectangle;
use image::{GenericImageView, RgbaImage, SubImage};

#[derive(Debug)]
pub struct GraphicSource {
    /// Region at atlas
    ///
    /// Defined at [`PackerProcessor`], shouldn't be changed manually
    pub atlas_region: Option<Rectangle<u32>>,

    /// Image buffer
    pub buffer: RgbaImage,

    /// Clipping region
    /// Usually to clip empty space to better packing
    pub region: Rectangle<u32>,
}

impl GraphicSource {
    pub fn new(buffer: RgbaImage, region: Rectangle<u32>) -> Self {
        Self {
            atlas_region: None,
            buffer,
            region,
        }
    }

    pub fn region_buffer_view(&self) -> SubImage<&RgbaImage> {
        self.buffer.view(
            self.region.x,
            self.region.y,
            self.region.width,
            self.region.height,
        )
    }
}
