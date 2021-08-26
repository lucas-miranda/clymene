use crate::graphics::{animation::Animation, Image};

pub enum Graphic {
    Empty,
    Image(Image),
    Animation(Animation),
}

impl From<Image> for Graphic {
    fn from(image: Image) -> Self {
        Graphic::Image(image)
    }
}

impl From<Animation> for Graphic {
    fn from(animation: Animation) -> Self {
        Graphic::Animation(animation)
    }
}
