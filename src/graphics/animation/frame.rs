use crate::graphics::GraphicSource;

#[derive(Debug)]
pub enum Frame {
    Empty,
    Contents {
        graphic_source: GraphicSource,
        duration: u32,
    },
}
