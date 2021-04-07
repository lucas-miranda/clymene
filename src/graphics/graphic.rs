use crate::graphics::{
    animation::Animation,
    Image
};

pub enum Graphic {
    Empty,
    Image(Image),
    Animation(Animation)
}
