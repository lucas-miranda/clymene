use crate::graphics::GraphicSource;

#[derive(Debug)]
pub struct Frame {
    pub graphic_source: GraphicSource,
    pub duration: u32,
}
