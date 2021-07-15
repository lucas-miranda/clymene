use crate::{
    graphics::GraphicSource,
    math::Size
};

pub trait Packer {
    fn name(&self) -> &str;
    fn execute(&self, atlas_min_size: Size<u32>, graphic_sources: &mut Vec<&mut GraphicSource>) -> Option<()>;
}
