use crate::logic::Logic;
use crate::num_traits::FromPrimitive;
use crate::KomaInf;
use std::ops::{BitAnd, BitOr, Not};

impl BitAnd for KomaInf {
    type Output = KomaInf;

    // rhs is the "right-hand side" of the expression `a & b`
    fn bitand(self, rhs: Self) -> Self::Output {
        // ビット演算で利用するときは 符号ビット は考えてないはずだから、型キャストは unsigned 型にしろだぜ☆（＾～＾）
        KomaInf::from_usize(self as usize & rhs as usize).unwrap()
    }
}

impl BitOr for KomaInf {
    type Output = KomaInf;

    // rhs is the "right-hand side" of the expression `a & b`
    fn bitor(self, rhs: Self) -> Self::Output {
        // ビット演算で利用するときは 符号ビット は考えてないはずだから、型キャストは unsigned 型にしろだぜ☆（＾～＾）
        KomaInf::from_usize(self as usize | rhs as usize).unwrap()
    }
}

impl Not for KomaInf {
    type Output = KomaInf;

    /// 0x01010101 を 0x10101010 にする操作。
    fn not(self) -> Self::Output {
        KomaInf::from_u8(!(self as u8)).unwrap()
    }
}

impl Logic for KomaInf {
    /// `if a & b` を仕方なく `if KomaInf::stood(a & b)` にするだけ。
    fn stood(&self) -> bool {
        *self != KomaInf::EMP
    }
}
