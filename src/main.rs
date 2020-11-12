extern crate num_derive;
extern crate num_traits;

pub mod koma_inf;
pub mod koma_info;
pub mod koma_moves;
pub mod kyokumen;
pub mod logic;
pub mod te;

use crate::num_traits::FromPrimitive;
use num_derive::FromPrimitive;
use rand::prelude::*;

// 非合法な手かどうか判定する関数です。
fn is_illegal(te: Te, te_num: TeNum, te_buf: &mut [Te; TE_LEN]) -> bool {
    // 要するに、手の一覧の中にあったら、
    for i in 0..te_num {
        if te == te_buf[i] {
            // Illegalではない、ということでfalseを返します。
            return false;
        }
    }
    // 手の一覧の中にない手は、違法な手＝指してはいけない手です。
    return true;
}

struct Sikou {}
impl Default for Sikou {
    fn default() -> Self {
        Sikou {}
    }
}
/// 簡単な思考ルーチンです。要するに、合法な手の中から、適当な手を乱数で選んで指すだけです。
impl Sikou {
    pub fn think(&self, s_or_e: KomaInf, k: &mut Kyokumen) -> Te {
        let mut te_buf: [Te; 600] = [Te::default(); 600];
        let te_num = k.make_legal_moves(s_or_e, &mut te_buf, &mut None);

        /*
        #ifdef _DEBUG
            // デバッグの際には、合法手を一覧表示します。
            for(int i=0;i<te_num;i++) {
                te_buf[i].print();
            }
            printf("\n");
        #endif
        */

        let r = random::<usize>() % te_num;
        return te_buf[r];
    }
}

fn main() {
    println!("Kifuwarabe's shogi with Lesserpyon");

    // 平手の初期配置です。見やすいでしょ？変換はその分複雑ですけど。
    use crate::KomaInf::*;
    let hirate_ban: [[KomaInf; 9]; 9] = [
        [EKY, EKE, EGI, EKI, EOU, EKI, EGI, EKE, EKY],
        [EMP, EHI, EMP, EMP, EMP, EMP, EMP, EKA, EMP],
        [EFU, EFU, EFU, EFU, EFU, EFU, EFU, EFU, EFU],
        [EMP, EMP, EMP, EMP, EMP, EMP, EMP, EMP, EMP],
        [EMP, EMP, EMP, EMP, EMP, EMP, EMP, EMP, EMP],
        [EMP, EMP, EMP, EMP, EMP, EMP, EMP, EMP, EMP],
        [SFU, SFU, SFU, SFU, SFU, SFU, SFU, SFU, SFU],
        [EMP, SKA, EMP, EMP, EMP, EMP, EMP, SHI, EMP],
        [SKY, SKE, SGI, SKI, SOU, SKI, SGI, SKE, SKY],
    ];
    // こちらは面倒でもEHIまで0を並べないといけません。
    let motigoma: [usize; KomaInf::EHI as usize + 1] = [
        // 空空空空空空空空空空空空空空空空空歩香桂銀金角飛王と杏圭全金馬龍空歩香桂銀金角飛
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // ０手目で、平手の局面で、持ち駒なしから開始しましょう。
    let mut k = Kyokumen::from_3(0, hirate_ban, motigoma);

    // これはまだ簡単な思考部なので、初期化も簡単です。
    let sikou = Sikou::default();

    // 将棋の局面で、最大の手数は５７９手だそうです。
    let mut te_buf: [Te; TE_LEN] = [Te::default(); TE_LEN];
    let mut te_num: usize;

    // 手前のプレイヤーから開始します。
    let mut s_or_e = KomaInf::Self_;

    // もしも合法手がなくなったら、詰み＝負けです。
    // 合法手がある間はゲームを継続します。
    te_num = k.make_legal_moves(s_or_e, &mut te_buf, &mut None);
    while te_num > 0 {
        k.print();
        let mut te: Te = Te::default();
        if s_or_e == KomaInf::Self_ {
            // 手を入力します。
            let mut buf: String = String::new();
            match std::io::stdin().read_line(&mut buf) {
                Ok(_n) => {}
                Err(why) => panic!("Failed to read line. / {}", why),
            };
            // 入力の方法:from,to,promote
            // ただし、歩を打つときはfromを01、香を打つときはfromを02…とする。
            // promoteは、成るときに*を付ける。
            let mut from: u8 = 0;
            let mut to: u8 = 0;
            let koma;
            let capture;
            let mut promote: [char; 2] = ['\0'; 2];

            let mut ss = 0;
            for ch in buf.chars() {
                match ss {
                    0 => from += (ch.to_digit(10).unwrap() * 10) as u8,
                    1 => from += ch.to_digit(10).unwrap() as u8,
                    2 => to += (ch.to_digit(10).unwrap() * 10) as u8,
                    3 => to += ch.to_digit(10).unwrap() as u8,
                    4 => promote[0] = ch,
                    _ => break,
                }
                ss += 1;
            }
            if ss < 2 {
                te_num = k.make_legal_moves(s_or_e, &mut te_buf, &mut None);
                continue;
            };
            if from < KomaInf::OU as u8 {
                koma = KomaInf::from_u8(KomaInf::Self_ as u8 | from).unwrap();
                from = 0;
            } else {
                koma = k.ban[from as usize];
            }
            capture = k.ban[to as usize];
            if ss == 3 && promote[0] == '*' {
                te = Te::from_5(from as USquare, to as USquare, koma, capture, 1);
            } else {
                te = Te::from_5(from as USquare, to as USquare, koma, capture, 0);
            }

            // 入力された手が、おかしかったら、
            if is_illegal(te, te_num, &mut te_buf) {
                // もう一回盤面を表示して入力しなおしです。
                print!("入力された手が異常です。入力しなおしてください。\n");
                te_num = k.make_legal_moves(s_or_e, &mut te_buf, &mut None);
                continue;
            }
        }
        if s_or_e == KomaInf::Enemy {
            te = sikou.think(s_or_e, &mut k);
        }

        te.print();
        k.move_(s_or_e, &te);
        if s_or_e == KomaInf::Self_ {
            s_or_e = KomaInf::Enemy;
        } else {
            s_or_e = KomaInf::Self_;
        }

        te_num = k.make_legal_moves(s_or_e, &mut te_buf, &mut None);
    }
    if s_or_e == KomaInf::Self_ {
        print!("後手の勝ち。\n");
    } else {
        print!("先手の勝ち。\n");
    }
    // return 0;
}

/// れさぴょん はしてないけど、盤のマス番地の型は明示しとこうぜ☆（＾～＾）
type USquare = usize;
type ISquare = isize;

/// 盤のデータの持ち方☆（＾～＾） パディングの説明とか どっかで読んどけだぜ☆（＾～＾）
const BAN_LEN: usize = 16 * (9 + 2);
/// * 王が持ち駒になることはないので、EHIまでで十分です。
const HAND_LEN: usize = KomaInf::EHI as usize + 1 as usize;

/// れさぴょん はしてないけど、手目の型は明示しとこうぜ☆（＾～＾）
type TeNum = usize;

/// C++ の れさぴょん は 手の配列の先頭アドレスを指す teBuf を使っていたが、
/// Rust で可変長を使うとだいぶ別物なんで、 とりあえず固定長の配列にしようぜ☆（＾～＾）？
const TE_LEN: usize = 600;

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
#[derive(Clone, Copy, FromPrimitive)]
pub enum KomaInfo {
    /// 何もないところ
    Empty = 0,
    // 成り駒につける目印（１ビット）
    Promoted = 1 << 3,
}
#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive)]
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
    pub ban: [KomaInf; BAN_LEN],

    /// 味方の駒の利き  
    ///
    /// # Tips
    ///
    /// * 敵の駒と自分の駒の利きは別々に保持します。
    pub control_s: [Kiki; BAN_LEN],
    /// 敵の駒の利き  
    pub control_e: [Kiki; BAN_LEN],

    /// 持ち駒の枚数  
    ///
    /// Hand[SFU]が１なら先手の持ち駒に歩が１枚、Hand[EKI]が３なら敵の持ち駒に金が３枚という要領です。  
    ///
    /// # Tips
    ///
    /// * 王が持ち駒になることはないので、EHIまでで十分です。
    pub hand: [usize; HAND_LEN],

    /// この局面の手数です。
    pub tesu: TeNum,

    /// 自玉の位置
    pub king_s: Kiki,

    /// 敵玉の位置
    pub king_e: Kiki,
}

// 手のクラス
#[derive(Clone, Copy)]
pub struct Te {
    // どこから・どこへはそれぞれ１Byteであらわせます。
    // 詳しくは局面クラスを参照して下さい。
    //
    // USquare ではなく u8 にします。
    pub from: u8,
    // USquare ではなく u8 にします。
    pub to: u8,
    // 動かした駒
    // KomaInf ではなく u8 にします。
    pub koma: u8,
    // 取った駒
    // KomaInf ではなく u8 にします。
    pub capture: u8,
    // 成/不成り
    pub promote: u8,
    // これは、手の生成の際に種別を用いたい時に使います（将来の拡張用）
    pub kind: u8,
    // その手の仮評価（手の価値）です
    pub value: i16,
}

pub struct KomaMoves {}
