use crate::graphics::Graphic;

pub struct GraphicOutput {
    pub graphics: Vec<Graphic>
}

impl GraphicOutput {
    pub fn new() -> Self {
        Self {
            graphics: Vec::new(),
        }
    }
}
