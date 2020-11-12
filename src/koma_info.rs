use crate::num_traits::FromPrimitive;
use crate::{KomaInf, KomaInfo};

/*
use std::ops::Not;
impl Not for KomaInfo {
    type Output = KomaInfo;

    /// 0x01010101 を 0x10101010 にする操作。
    fn not(self) -> Self::Output {
        KomaInfo::from_u8(!(self as u8)).unwrap()
    }
}
*/

impl KomaInfo {
    pub fn to_koma_inf(&self) -> KomaInf {
        KomaInf::from_u8(*self as u8).unwrap()
    }
}
