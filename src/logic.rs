//! C++ の書き方でできて、 Rust の書き方でできないことを埋めるためのものだぜ☆（＾～＾）

pub trait Logic {
    /// `if a & b` を仕方なく `if (a & b).stood` にするだけ。
    fn stood(&self) -> bool;
}
impl Logic for usize {
    /// `if a & b` を仕方なく `if (a & b).stood` にするだけ。
    fn stood(&self) -> bool {
        *self != 0
    }
}
