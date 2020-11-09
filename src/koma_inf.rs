use crate::KomaInf;
use std::ops::{BitAnd, BitOr};

impl BitAnd for KomaInf {
    type Output = isize;

    // rhs is the "right-hand side" of the expression `a & b`
    fn bitand(self, rhs: Self) -> Self::Output {
        self as isize & rhs as isize
    }
}

impl BitOr for KomaInf {
    type Output = isize;

    // rhs is the "right-hand side" of the expression `a & b`
    fn bitor(self, rhs: Self) -> Self::Output {
        self as isize | rhs as isize
    }
}
