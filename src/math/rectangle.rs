use std::cmp::PartialOrd;

use num_traits::{
    cast::{
        cast,
        NumCast
    },
    Num,
    sign::Unsigned
};

use super::Size;

#[derive(Default, Clone, Debug)]
pub struct Rectangle<T: Num + NumCast + PartialOrd + Copy> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T
}

impl<T: Num + NumCast + PartialOrd + Copy> Rectangle<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height
        }
    }

    pub fn with<S: Num + NumCast + PartialOrd + Copy>(x: S, y: S, width: S, height: S) -> Option<Self> {
        Some(Self::new(
            cast::<S, T>(x)?, 
            cast::<S, T>(y)?, 
            cast::<S, T>(width)?, 
            cast::<S, T>(height)?
        ))
    }

    pub fn try_size<S: Unsigned + NumCast + PartialOrd + Copy>(&self) -> Option<Size<S>> {
        Size::with(self.width, self.height)
    }

    pub fn left(&self) -> T {
        self.x
    }

    pub fn top(&self) -> T {
        self.y
    }

    pub fn right(&self) -> T {
        self.x + self.width
    }

    pub fn bottom(&self) -> T {
        self.y + self.height
    }
}

impl<T: Unsigned + NumCast + PartialOrd + Copy> Rectangle<T> {
    pub fn size(&self) -> Size<T> {
        Size::new(self.width, self.height)
    }

    pub fn fit_size(&self, size: &Size<T>) -> bool {
        self.width.ge(&size.width) && self.height.ge(&size.height)
    }
}
