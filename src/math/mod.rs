use num_traits::{Num, NumCast};
use std::ops::{BitAnd, BitOrAssign, Shr};

mod rectangle;
mod size;

pub use rectangle::Rectangle;
pub use size::Size;

pub fn is_power_2<N: Num + BitAnd<Output = N> + Copy>(n: N) -> bool {
    (n & (n - N::one())).is_zero()
}

pub fn ceil_power_2<N>(n: N) -> N
where
    N: Num + NumCast + BitAnd<Output = N> + BitOrAssign + Shr<Output = N> + Copy,
{
    if n.is_zero() {
        return n;
    }

    let mut n = n - N::one();
    n |= n >> num_traits::cast(1).unwrap();
    n |= n >> num_traits::cast(2).unwrap();
    n |= n >> num_traits::cast(4).unwrap();
    n |= n >> num_traits::cast(8).unwrap();
    n |= n >> num_traits::cast(16).unwrap();

    n + N::one()
}
