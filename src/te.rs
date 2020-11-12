use crate::koma_moves::KOMA_STR2;
use crate::KomaInf;
use crate::Te;
use crate::USquare;

/// Teを空のデータで初期化したい時のためのコンストラクタです。
impl Default for Te {
    fn default() -> Self {
        Te {
            from: 0,
            to: 0,
            koma: KomaInf::EMP as u8,
            capture: KomaInf::EMP as u8,
            promote: 0,
            kind: 0,
            value: 0,
        }
    }
}

impl Te {
    pub fn from_4(f: USquare, t: USquare, k: KomaInf, c: KomaInf) -> Self {
        Te::from_7(f, t, k, c, 0, 0, 0)
    }
    pub fn from_5(f: USquare, t: USquare, k: KomaInf, c: KomaInf, p: u8) -> Self {
        Te::from_7(f, t, k, c, p, 0, 0)
    }
    pub fn from_7(f: USquare, t: USquare, ko: KomaInf, c: KomaInf, p: u8, ki: u8, v: i16) -> Self {
        Te {
            from: f as u8,
            to: t as u8,
            koma: ko as u8,
            capture: c as u8,
            promote: p,
            kind: ki,
            value: v,
        }
    }
    pub fn is_null(&self) -> bool {
        return self.from == 0 && self.to == 0;
    }
    /// 手を表示したい時に使います。
    pub fn print(&self) {
        print!("{:0>2}", self.to);
        print!("{:0>2}", KOMA_STR2[self.koma as usize]);
        if self.promote != 0 {
            print!("成");
        }
        if self.from < KomaInf::OU as u8 {
            print!("打");
        } else {
            print!("({:0>2x})", self.from);
        }

        if !(self.promote != 0) {
            print!("　");
        }
    }
    /*
    /// 同上
    fn fprint(FILE *fp)
    {

    }
    */
}

impl PartialEq for Te {
    /// 手の同一性を比較したいときに使います。KindやValueが違っても同じ手です。
    fn eq(&self, other: &Self) -> bool {
        return other.from == self.from
            && other.to == self.to
            && other.koma == self.koma
            && other.promote == self.promote;
    }
}
