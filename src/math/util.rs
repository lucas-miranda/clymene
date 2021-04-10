use std::cmp::PartialOrd;

pub fn min<'a, T: PartialOrd>(a: &'a T, b: &'a T) -> &'a T {
    if a.le(b) {
        a
    } else {
        b
    }
}

pub fn max<'a, T: PartialOrd>(a: &'a T, b: &'a T) -> &'a T {
    if a.ge(b) {
        a
    } else {
        b
    }
}
