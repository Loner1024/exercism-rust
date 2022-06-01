// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let quotient = dividend / divisor;
    let remainder = dividend - (quotient * divisor);

    (quotient, remainder)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.enumerate()
        .filter_map(|(i, v)| if i % 2 == 0 { Some(v) } else { None })
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        abs(self.0) + abs(self.1)
    }
}

fn abs(i: i16) -> i16 {
    if i < 0 {
        -i
    } else {
        i
    }
}
