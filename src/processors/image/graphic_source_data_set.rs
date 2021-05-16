use crate::{
    processors::image::GraphicSourceData,
    math::Size
};

pub struct GraphicSourceDataSet {
    pub sources: Vec<GraphicSourceData>,
    pub dimensions: Option<Size<u32>>
}

impl GraphicSourceDataSet {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            dimensions: None
        }
    }
}
