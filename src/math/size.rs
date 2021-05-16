use std::{
    cmp::PartialOrd,
    fmt
};

use num_traits::{
    cast::{
        cast,
        NumCast
    },
    identities::zero,
    Num,
    sign::Unsigned
};

use super::Rectangle;

#[derive(Default, Clone, Debug)]
pub struct Size<T: Unsigned + NumCast + PartialOrd + Copy> {
    pub width: T,
    pub height: T
}

impl<T: Unsigned + NumCast + PartialOrd + Copy> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Self {
            width,
            height
        }
    }

    pub fn with<S: Num + NumCast + PartialOrd + Copy>(width: S, height: S) -> Option<Self> {
        let zero_value = zero::<S>();

        let w = if width.lt(&zero_value) {
            Some(zero::<T>())
        } else {
            cast::<S, T>(width)
        }?;

        let h = if height.lt(&zero_value) {
            Some(zero::<T>())
        } else {
            cast::<S, T>(height)
        }?;

        Some(Self::new(w, h))
    }
}

impl<T: Unsigned + NumCast + PartialOrd + Copy> From<Size<T>> for Rectangle<T> {
    fn from(size: Size<T>) -> Self {
        Rectangle::new(zero::<T>(), zero::<T>(), size.width, size.height)
    }
}

impl<T: Unsigned + NumCast + PartialOrd + Copy + fmt::Display> fmt::Display for Size<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.width, self.height)
    }
}
