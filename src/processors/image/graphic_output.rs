use crate::graphics::{
    animation::Animation,
    Image
};

pub struct GraphicOutput {
    pub images: Vec<Image>,
    pub animations: Vec<Animation>
}

impl GraphicOutput {
    pub fn new() -> Self {
        Self {
            images: Vec::new(),
            animations: Vec::new()
        }
    }
}
