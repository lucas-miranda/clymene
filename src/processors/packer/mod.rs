mod row_tight_packer;
mod error;
mod packer_processor;

pub use row_tight_packer::RowTightPacker;
pub use error::Error;
pub use packer_processor::PackerProcessor;

use crate::{graphics::GraphicSource, math::Size};

pub trait Packer {
    fn name(&self) -> &str;
    fn execute(
        &self,
        atlas_min_size: Size<u32>,
        graphic_sources: &mut Vec<&mut GraphicSource>,
    ) -> Option<()>;
}
