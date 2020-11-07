pub mod kyokumen;

fn main() {
    println!("Kifuwarabe's shogi with Usapyon");

    let kyokumen = Kyokumen::default();
}

/// Empty=0,
/// EMP=0,
/// のような書き方は Rust言語では already exists になるので、名前の長い方を この列挙型に分ける。
pub enum KomaInfo {
    /// 何もないところ
    Empty = 0,
    // 成り駒につける目印（１ビット）
    Promoted = 1 << 3,
}
#[derive(Clone, Copy)]
pub enum KomaInf {
    /// ３文字も準備しておくとソースが見やすいので（笑）
    EMP = 0,

    // 駒をあらわす数値
    FU = 1,
    KY = 2,
    KE = 3,
    GI = 4,
    KI = 5,
    KA = 6,
    HI = 7,
    OU = 8,
    TO = KomaInf::FU as isize + KomaInfo::Promoted as isize,
    NY = KomaInf::KY as isize + KomaInfo::Promoted as isize,
    NK = KomaInf::KE as isize + KomaInfo::Promoted as isize,
    NG = KomaInf::GI as isize + KomaInfo::Promoted as isize,
    UM = KomaInf::KA as isize + KomaInfo::Promoted as isize,
    RY = KomaInf::HI as isize + KomaInfo::Promoted as isize,
    /// 自分自身の駒につける目印（１ビット）
    Self_ = 1 << 4,

    /// 敵の駒につける目印(１ビット)
    Enemy = 1 << 5,

    /// 敵も味方も進めないところ（盤の外）の目印
    Wall = KomaInf::Self_ as isize + KomaInf::Enemy as isize,
    // 実際の駒
    SFU = KomaInf::Self_ as isize + KomaInf::FU as isize, //味方の歩
    STO = KomaInf::Self_ as isize + KomaInf::TO as isize, //味方のと金
    SKY = KomaInf::Self_ as isize + KomaInf::KY as isize, //味方の香車
    SNY = KomaInf::Self_ as isize + KomaInf::NY as isize, //味方の成り香
    SKE = KomaInf::Self_ as isize + KomaInf::KE as isize, //味方の桂馬
    SNK = KomaInf::Self_ as isize + KomaInf::NK as isize, //味方の成り桂
    SGI = KomaInf::Self_ as isize + KomaInf::GI as isize, //味方の銀
    SNG = KomaInf::Self_ as isize + KomaInf::NG as isize, //味方の成り銀
    SKI = KomaInf::Self_ as isize + KomaInf::KI as isize, //味方の金
    SKA = KomaInf::Self_ as isize + KomaInf::KA as isize, //味方の角
    SUM = KomaInf::Self_ as isize + KomaInf::UM as isize, //味方の馬
    SHI = KomaInf::Self_ as isize + KomaInf::HI as isize, //味方の飛車
    SRY = KomaInf::Self_ as isize + KomaInf::RY as isize, //味方の龍
    SOU = KomaInf::Self_ as isize + KomaInf::OU as isize, //味方の玉

    EFU = KomaInf::Enemy as isize + KomaInf::FU as isize, //敵の歩
    ETO = KomaInf::Enemy as isize + KomaInf::TO as isize, //敵のと金
    EKY = KomaInf::Enemy as isize + KomaInf::KY as isize, //敵の香車
    ENY = KomaInf::Enemy as isize + KomaInf::NY as isize, //敵の成り香
    EKE = KomaInf::Enemy as isize + KomaInf::KE as isize, //敵の桂馬
    ENK = KomaInf::Enemy as isize + KomaInf::NK as isize, //敵の成り桂
    EGI = KomaInf::Enemy as isize + KomaInf::GI as isize, //敵の銀
    ENG = KomaInf::Enemy as isize + KomaInf::NG as isize, //敵の成り銀
    EKI = KomaInf::Enemy as isize + KomaInf::KI as isize, //敵の金
    EKA = KomaInf::Enemy as isize + KomaInf::KA as isize, //敵の角
    EUM = KomaInf::Enemy as isize + KomaInf::UM as isize, //敵の馬
    EHI = KomaInf::Enemy as isize + KomaInf::HI as isize, //敵の飛車
    ERY = KomaInf::Enemy as isize + KomaInf::RY as isize, //敵の龍
    EOU = KomaInf::Enemy as isize + KomaInf::OU as isize, //敵の玉
}

/// 局面。
pub struct Kyokumen {
    /// 桂馬の利きが盤外のさらに外にはみ出すことを考慮して設けてある。
    ///
    /// * `KomaInf::Wall` - banpaddingの中は、常にWALLである。
    pub banpadding: [KomaInf; 16],

    /// 盤面。
    ///
    /// * `16 *` - 高速化のためには、１次元配列として、演算としては＊１６など２の階乗倍が使えることが望ましい。
    pub ban: [KomaInf; 16 * (9 + 2)],

    /// 持ち駒の枚数。
    pub hand: [usize; KomaInf::EHI as usize + 1 as usize],

    /// 方向を示す定数。
    pub direct: [isize; 12],

    /// その方向に動けるか？その方向に飛んで動くものは入れてはいけない。
    pub can_move: [[isize; 64]; 12],
    /// その方向に飛んで動くことが出来るか？
    /// 飛車角香車と龍と馬しかそういう駒はない
    pub can_jump: [[isize; 64]; 12],
}
