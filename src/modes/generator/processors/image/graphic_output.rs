use crate::graphics::Graphic;

pub struct GraphicOutput {
    pub graphics: Vec<Graphic>,
    requested: bool,
}

impl GraphicOutput {
    pub fn new() -> Self {
        Self {
            graphics: Vec::new(),
            requested: false,
        }
    }

    pub fn request(&mut self) {
        self.requested = true;
    }

    pub fn is_requested(&self) -> bool {
        self.requested
    }
}
