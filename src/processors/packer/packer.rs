use crate::{
    graphics::Image,
    math::Size
};

pub trait Packer {
    fn execute(&self, atlas_min_size: Size<u32>, source_images: &mut Vec<&mut Image>) -> Option<()>;
}
