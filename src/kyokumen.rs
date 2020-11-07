//! 局面の実装。

use crate::{KomaInf, Kyokumen};

impl Default for Kyokumen {
    fn default() -> Self {
        Kyokumen {
            banpadding: [KomaInf::Wall; 16],
            ban: [KomaInf::EMP; 16 * (9 + 2)],
            hand: [0; KomaInf::EHI as usize + 1 as usize],
        }
    }
}
