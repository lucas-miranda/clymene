use std::cmp::PartialOrd;

pub fn max<'a, T: PartialOrd>(a: &'a T, b: &'a T) -> &'a T {
    if a.ge(b) {
        a
    } else {
        b
    }
}
