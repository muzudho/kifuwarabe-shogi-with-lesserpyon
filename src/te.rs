use crate::KomaInf;
use crate::Te;

/// Teを空のデータで初期化したい時のためのコンストラクタです。
impl Default for Te {
    fn default() -> Self {
        Te {
            from: 0,
            to: 0,
            koma: KomaInf::EMP,
            capture: KomaInf::EMP,
            promote: 0,
            kind: 0,
            value: 0,
        }
    }
}

impl Te {
    pub fn from_7(f: u8, t: u8, ko: KomaInf, c: KomaInf, p: u8, ki: u8, v: i16) -> Self {
        Te {
            from: f,
            to: t,
            koma: ko,
            capture: c,
            promote: p,
            kind: ki,
            value: v,
        }
    }
    pub fn from_4(f: u8, t: u8, k: KomaInf, c: KomaInf) -> Self {
        Te {
            from: f,
            to: t,
            koma: k,
            capture: c,
            promote: 0,
            kind: 0,
            value: 0,
        }
    }
    pub fn is_null(&self) -> bool {
        return self.from == 0 && self.to == 0;
    }
    /// 手を表示したい時に使います。
    fn print(&self) {
        // FPrint(stdout);
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