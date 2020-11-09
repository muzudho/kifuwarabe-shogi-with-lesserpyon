extern crate num_derive;
extern crate num_traits;

pub mod koma_inf;
pub mod kyokumen;
pub mod te;

use num_derive::FromPrimitive;
// use num_traits::FromPrimitive;

fn main() {
    println!("Kifuwarabe's shogi with Lesserpyon");

    let _kyokumen = Kyokumen::default();
}

/// れさぴょん はしてないけど、盤のマス番地の型は明示しとこうぜ☆（＾～＾）
type USquare = usize;
type ISquare = isize;

/// 盤のデータの持ち方☆（＾～＾） パディングの説明とか どっかで読んどけだぜ☆（＾～＾）
const BAN_LEN: usize = 16 * (9 + 2);

/// れさぴょん はしてないけど、手目の型は明示しとこうぜ☆（＾～＾）
type TeNum = usize;

/// C++ の れさぴょん は 手の配列の先頭アドレスを指す teBuf を使っていたが、
/// Rust で可変長を使うとだいぶ別物なんで、 とりあえず固定長の配列にしようぜ☆（＾～＾）？
const TE_LEN: usize = 512;

/// Pin.
///
/// 玉と敵駒の間にある合い駒は、ピンしている敵駒の方向を覚えておくぜ☆（＾～＾）
///
/// C++ は配列のサイズを指定しなくても ポインターで先頭アドレス指すだけでいいんだが、
/// Rust はそうもいかないんで サイズを指定できるようにしておこうぜ☆（＾～＾）？
type Pin = [ISquare; BAN_LEN];

/// Empty=0,
/// EMP=0,
/// のような書き方は Rust言語では already exists になるので、名前の長い方を この列挙型に分ける。
pub enum KomaInfo {
    /// 何もないところ
    Empty = 0,
    // 成り駒につける目印（１ビット）
    Promoted = 1 << 3,
}
#[derive(Clone, Copy, PartialEq, FromPrimitive)]
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

/// 方向を示す定数。
const DIRECT: [ISquare; 12] = [17, 1, -15, 16, -16, 15, -1, -17, 14, -18, 18, -14];

/// 利き。
type Kiki = USquare;

/// 局面。
pub struct Kyokumen {
    /// メモリ上の隙間  
    ///
    /// 桂馬の利きがbanからはみ出すので、はみ出す分を確保しておきます。  
    /// C++では、構造体の内部の変数の並び順は宣言した順になることを利用しています。  
    /// 普通はあまり使わない「汚い」テクニックですけど、こういうテクニックもあるということで。  
    ///
    /// # Remarks
    ///
    /// * `KomaInf::Wall` - banpaddingの中は、常にWALLである。
    pub banpadding: [KomaInf; 16],

    /// 盤面  
    ///
    /// 2次元配列を使うと遅いので、１次元配列を使います。また、掛け算の際に、＊９とかを用いるよりも、  
    /// 2の階乗を掛け算に使うと掛け算が早くなるので、＊１６を使います。  
    /// 駒の位置は、例えば７七なら、７＊１６＋七のようにあらわします。  
    /// つまり、７七なら１６進数で0x77になるわけです。  
    ///
    /// # Remarks
    ///
    /// * `16 *` - 高速化のためには、１次元配列として、演算としては＊１６など２の階乗倍が使えることが望ましい。
    pub ban: [KomaInf; 16 * (9 + 2)],

    /// 味方の駒の利き  
    ///
    /// # Tips
    ///
    /// * 敵の駒と自分の駒の利きは別々に保持します。
    pub control_s: [Kiki; 16 * 11],
    /// 敵の駒の利き  
    pub control_e: [Kiki; 16 * 11],

    /// 持ち駒の枚数  
    ///
    /// Hand[SFU]が１なら先手の持ち駒に歩が１枚、Hand[EKI]が３なら敵の持ち駒に金が３枚という要領です。  
    ///
    /// # Tips
    ///
    /// * 王が持ち駒になることはないので、EHIまでで十分です。
    pub hand: [usize; KomaInf::EHI as usize + 1 as usize],

    /// この局面の手数です。
    pub tesu: TeNum,

    /// 自玉の位置
    pub king_s: Kiki,

    /// 敵玉の位置
    pub king_e: Kiki,

    /// その方向に動けるか？その方向に飛んで動くものは入れてはいけない。
    pub can_move: [[isize; 64]; 12],
    /// その方向に飛んで動くことが出来るか？
    /// 飛車角香車と龍と馬しかそういう駒はない
    pub can_jump: [[isize; 64]; 12],
}

// 手のクラス
pub struct Te {
    // どこから・どこへはそれぞれ１Byteであらわせます。
    // 詳しくは局面クラスを参照して下さい。
    pub from: u8,
    pub to: u8,
    // 動かした駒
    pub koma: KomaInf,
    // 取った駒
    pub capture: KomaInf,
    // 成/不成り
    pub promote: u8,
    // これは、手の生成の際に種別を用いたい時に使います（将来の拡張用）
    pub kind: u8,
    // その手の仮評価（手の価値）です
    pub value: i16,
}
