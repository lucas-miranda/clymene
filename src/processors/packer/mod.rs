mod column_tight_packer;
pub use column_tight_packer::ColumnTightPacker;

mod error;
pub use error::Error;

mod packer_processor;
pub use packer_processor::PackerProcessor;

use crate::{
    graphics::GraphicSource,
    math::Size
};

pub trait Packer {
    fn name(&self) -> &str;
    fn execute(&self, atlas_min_size: Size<u32>, graphic_sources: &mut Vec<&mut GraphicSource>) -> Option<()>;
}
