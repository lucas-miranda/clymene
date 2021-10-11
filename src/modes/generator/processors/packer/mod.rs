mod packer_processor;
mod row_tight_packer;

pub use packer_processor::PackerProcessor;
pub use row_tight_packer::RowTightPacker;

use crate::{graphics::GraphicSource, math::Size};

pub trait Packer {
    fn name(&self) -> &str;
    fn execute(
        &self,
        atlas_min_size: Size<u32>,
        graphic_sources: &mut Vec<&mut GraphicSource>,
    ) -> Option<()>;
}
