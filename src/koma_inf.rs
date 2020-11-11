use crate::num_traits::FromPrimitive;
use crate::KomaInf;
use std::ops::{BitAnd, BitOr};

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

impl KomaInf {
    /// `if a & b` を仕方なく `if KomaInf::stood(a & b)` にするだけ。
    pub fn stood(a: Self) -> bool {
        a != KomaInf::EMP
    }
}
