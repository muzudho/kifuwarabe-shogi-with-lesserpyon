//! 局面の実装。

use crate::koma_moves::CAN_JUMP;
use crate::koma_moves::CAN_MOVE;
use crate::koma_moves::CAN_PROMOTE;
use crate::koma_moves::DIRECT;
use crate::koma_moves::KOMA_STR;
use crate::koma_moves::KOMA_STR2;
use crate::logic::*;
use crate::ISquare;
use crate::Kiki;
use crate::Pin;
use crate::Te;
use crate::TeNum;
use crate::USquare;
use crate::BAN_LEN;
use crate::HAND_LEN;
use crate::TE_LEN;
use crate::{KomaInf, KomaInfo, Kyokumen};
use num_traits::FromPrimitive;

impl Default for Kyokumen {
    fn default() -> Self {
        Kyokumen {
            banpadding: [KomaInf::Wall; 16],
            ban: [KomaInf::EMP; BAN_LEN],
            control_s: [0; BAN_LEN],
            control_e: [0; BAN_LEN],
            hand: [0; HAND_LEN],
            tesu: 0,
            king_s: 0,
            king_e: 0,
        }
    }
}

/// C++からRustに翻訳するときに 可読性が低くなったところに使うぜ☆（＾～＾）  
/// きふわらべのお父ん が追加した☆（＾～＾）  
/// オリジナルの れさぴょん には無いぜ☆（＾～＾）  
impl Kyokumen {
    /// Is self.
    pub fn is_s(&self, sq: USquare) -> bool {
        self.ban[sq] & KomaInf::Self_ != KomaInf::EMP
    }
    /// Is enemy.
    pub fn is_e(&self, sq: USquare) -> bool {
        self.ban[sq] & KomaInf::Enemy != KomaInf::EMP
    }
    /// Is intersect enemy.
    pub fn is_control_e(&self, sq: USquare) -> bool {
        self.control_e[sq] != 0
    }
    /// Is intersect self.
    pub fn is_control_s(&self, sq: USquare) -> bool {
        self.control_s[sq] != 0
    }
    /// Exists.
    pub fn is_exists(&self, sq: USquare) -> bool {
        self.ban[sq] != KomaInf::EMP
    }
    /// Exists.
    pub fn is_exists_s_or_e(&self, sq: USquare, s_or_e: KomaInf) -> bool {
        self.ban[sq] & s_or_e != KomaInf::EMP
    }
    /// # Parameters
    ///
    /// * `dir` - Direction.
    /// * `sq` - Square.
    pub fn can_move(&self, dir: usize, sq: ISquare) -> bool {
        CAN_MOVE[dir][self.ban[(sq - DIRECT[dir]) as usize] as usize] != 0
    }
    pub fn can_jump(&self, dir: usize, sq: USquare) -> bool {
        CAN_MOVE[dir][self.ban[sq] as usize] != 0
    }
    pub fn can_promote(&self, sq: USquare) -> bool {
        CAN_PROMOTE[self.ban[sq] as usize] != 0
    }
    pub fn get_koma_by_offset_sq(&self, absolute_sq: USquare, relative_sq: ISquare) -> KomaInf {
        self.ban[Kyokumen::get_offset_sq(absolute_sq, relative_sq)]
    }
    /// プラスではなく、マイナスすることに注意☆（＾～＾）
    pub fn get_offset_sq(absolute_sq: USquare, relative_sq: ISquare) -> USquare {
        (absolute_sq as ISquare - DIRECT[relative_sq as usize]) as USquare
    }
}

impl Kyokumen {
    // TODO pub Kyokumen() {}
    pub fn from_3(tesu: usize, board: [[KomaInf; 9]; 9], motigoma: [usize; HAND_LEN]) -> Self {
        // self.
        let mut s = Kyokumen::default();

        s.king_s = 0;
        s.king_e = 0;
        s.tesu = tesu;
        // 盤面をWALL（壁）で埋めておきます。
        s.banpadding = [KomaInf::Wall; 16];
        s.ban = [KomaInf::Wall; BAN_LEN];
        // boardで与えられた局面を設定します。
        for dan in 1..=9 {
            for suji in (0x10..=0x90).step_by(0x10) {
                // 将棋の筋は左から右なので、配列の宣言と逆になるため、筋はひっくり返さないとなりません。
                s.ban[suji + dan] = board[dan - 1][9 - suji / 0x10];
                if s.ban[suji + dan] == KomaInf::SOU {
                    s.king_s = suji + dan;
                }
                if s.ban[suji + dan] == KomaInf::EOU {
                    s.king_e = suji + dan;
                }
            }
        }
        // 持ち駒はそのまま利用します。
        for i in 0..=KomaInf::EHI as usize {
            s.hand[i] = motigoma[i];
        }
        // self.control_s/controlEを初期化します。
        s.init_control();
        s
    }

    /// １章で追加。controlS,controlEの初期化。
    fn init_control(&mut self) {
        // let dan: usize;
        // let suji: usize;
        let mut i;
        let mut j: USquare;
        let mut b;
        let mut bj;
        self.control_s = [0; BAN_LEN];
        self.control_e = [0; BAN_LEN];
        for suji in (0x10..=0x90).step_by(0x10) {
            for dan in 1..=9 {
                if (self.ban[suji + dan] & KomaInf::Enemy).stood() {
                    //敵の駒
                    //駒の効きを追加する
                    i = 0;
                    b = 1;
                    bj = 1 << 16;
                    while i < 12 {
                        if CAN_JUMP[i][self.ban[dan + suji] as usize] != 0 {
                            j = dan + suji;
                            while {
                                j = (j as ISquare + DIRECT[i]) as USquare;
                                self.control_e[j] |= bj;
                                self.ban[j] == KomaInf::EMP
                            } {}
                        } else if CAN_MOVE[i][self.ban[dan + suji] as usize] != 0 {
                            self.control_e[((dan + suji) as isize + DIRECT[i]) as usize] |= b;
                        }
                        i += 1;
                        b <<= 1;
                        bj <<= 1;
                    }
                } else if (self.ban[suji + dan] & KomaInf::Self_).stood() {
                    //味方の駒が有る
                    //駒の効きを追加する
                    i = 0;
                    b = 1;
                    bj = 1 << 16;
                    while i < 12 {
                        if CAN_JUMP[i][self.ban[dan + suji] as usize] != 0 {
                            j = dan + suji;
                            while {
                                j = (j as ISquare + DIRECT[i]) as USquare;
                                self.control_s[j] |= bj;
                                self.ban[j] == KomaInf::EMP
                            } {}
                        } else if CAN_MOVE[i][self.ban[dan + suji] as usize] != 0 {
                            self.control_s[((dan + suji) as isize + DIRECT[i]) as usize] |= b;
                        }
                        i += 1;
                        b <<= 1;
                        bj <<= 1;
                    }
                }
            }
        }
    }

    /// TODO 手で局面を進める
    pub fn move_(&mut self, s_or_e: KomaInf, te: &Te) {
        let mut i;
        let mut j: USquare;
        let mut b;
        let mut bj;
        if te.from > 0x10 {
            // 元いた駒のコントロールを消す
            let mut dir = 0;
            b = 1;
            bj = 1 << 16;
            while dir < 12 {
                if s_or_e == KomaInf::Self_ {
                    self.control_s[(te.from as isize + DIRECT[dir]) as usize] &= !b;
                // binary反転
                } else {
                    self.control_e[(te.from as isize + DIRECT[dir]) as usize] &= !b;
                    // binary反転
                }
                if CAN_JUMP[dir as usize][te.koma as usize] != 0 {
                    j = te.from as USquare;
                    while {
                        j = (j as isize + DIRECT[dir]) as usize;
                        if s_or_e == KomaInf::Self_ {
                            self.control_s[j] &= !bj; // binary反転
                        } else {
                            self.control_e[j] &= !bj; // binary反転
                        }
                        self.ban[j] == KomaInf::EMP
                    } {}
                }
                dir += 1;
                b <<= 1;
                bj <<= 1;
            }
            // 元いた位置は空白になる
            self.ban[te.from as usize] = KomaInf::EMP;
            // 飛び利きを伸ばす
            i = 0;
            bj = 1 << 16;
            while i < 8 {
                let dir: ISquare = DIRECT[i];
                if (self.control_s[te.from as usize] & bj).stood() {
                    j = te.from as USquare;
                    while {
                        j = (j as ISquare + dir) as USquare;
                        self.control_s[j] |= bj;
                        self.ban[j] == KomaInf::EMP
                    } {}
                }
                if (self.control_e[te.from as usize] & bj).stood() {
                    j = te.from as USquare;
                    while {
                        j = (j as ISquare + dir) as USquare;
                        self.control_e[j] |= bj;
                        self.ban[j] == KomaInf::EMP
                    } {}
                }
                i += 1;
                bj <<= 1;
            }
        } else {
            // 持ち駒から一枚減らす
            self.hand[te.koma as usize] -= 1;
        }
        if self.ban[te.to as usize] != KomaInf::EMP {
            // 相手の駒を持ち駒にする。
            // 持ち駒にする時は、成っている駒も不成りに戻す。（&~PROMOTED）
            self.hand[(s_or_e
                | (self.ban[te.to as usize]
                    & !KomaInfo::Promoted.to_koma_inf()
                    & !KomaInf::Self_
                    & !KomaInf::Enemy)) as usize] += 1;
            //取った駒の効きを消す
            i = 0;
            b = 1;
            bj = 1 << 16;
            while i < 12 {
                let dir = DIRECT[i];
                if CAN_JUMP[i][self.ban[te.to as usize] as usize] != 0 {
                    j = te.to as USquare;
                    while {
                        j = (j as ISquare + dir) as USquare;
                        if s_or_e == KomaInf::Self_ {
                            self.control_e[j] &= !bj; // binary反転
                        } else {
                            self.control_s[j] &= !bj; // binary反転
                        }
                        self.ban[j] == KomaInf::EMP
                    } {}
                } else {
                    j = (te.to as ISquare + dir) as USquare;
                    if s_or_e == KomaInf::Self_ {
                        self.control_e[j] &= !b; // binary反転
                    } else {
                        self.control_s[j] &= !b; // binary反転
                    }
                }
                i += 1;
                b <<= 1;
                bj <<= 1;
            }
        } else {
            // 移動先で遮った飛び利きを消す
            i = 0;
            bj = 1 << 16;
            while i < 8 {
                let dir = DIRECT[i];
                if (self.control_s[te.to as usize] & bj).stood() {
                    j = te.to as USquare;
                    while {
                        j = (j as ISquare + dir) as USquare;
                        self.control_s[j] &= !bj; // binary反転
                        self.ban[j] == KomaInf::EMP
                    } {}
                }
                if (self.control_e[te.to as usize] & bj).stood() {
                    j = te.to as USquare;
                    while {
                        j = (j as ISquare + dir) as USquare;
                        self.control_e[j] &= !bj; // binary反転
                        self.ban[j] == KomaInf::EMP
                    } {}
                }

                i += 1;
                bj <<= 1;
            }
        }
        if te.promote != 0 {
            self.ban[te.to as usize] =
                KomaInf::from_u8(te.koma | KomaInfo::Promoted as u8).unwrap();
        } else {
            self.ban[te.to as usize] = KomaInf::from_u8(te.koma).unwrap();
        }
        // 移動先の利きをつける
        i = 0;
        b = 1;
        bj = 1 << 16;
        while i < 12 {
            if CAN_JUMP[i][self.ban[te.to as usize] as usize] != 0 {
                j = te.to as USquare;
                while {
                    j = (j as ISquare + DIRECT[i]) as USquare;
                    if s_or_e == KomaInf::Self_ {
                        self.control_s[j] |= bj;
                    } else {
                        self.control_e[j] |= bj;
                    }
                    self.ban[j] == KomaInf::EMP
                } {}
            } else if CAN_MOVE[i][self.ban[te.to as usize] as usize] != 0 {
                if s_or_e == KomaInf::Self_ {
                    self.control_s[(te.to as ISquare + DIRECT[i]) as usize] |= b;
                } else {
                    self.control_e[(te.to as ISquare + DIRECT[i]) as usize] |= b;
                }
            }
            i += 1;
            b <<= 1;
            bj <<= 1;
        }
        // 王様の位置は覚えておく。
        if te.koma == KomaInf::SOU as u8 {
            self.king_s = te.to as Kiki;
        }
        if te.koma == KomaInf::EOU as u8 {
            self.king_e = te.to as Kiki;
        }

        self.tesu += 1;
    }
    /// ピン（動かすと王を取られてしまうので動きが制限される駒）の状態を設定する
    pub fn make_pin_inf(&self, pin: &mut Pin) {
        // int i;
        // ピン情報を設定する
        for sq in 0x11..=0x99 {
            // 0はピンされていない、という意味
            pin[sq] = 0;
        }
        if self.king_s != 0 {
            //自玉が盤面にある時のみ有効
            for i in 0..8 {
                let p = self.search(self.king_s, DIRECT[i]);
                if self.ban[p] != KomaInf::Wall && !self.is_e(p) {
                    //味方の駒が有る
                    if (self.control_e[p] & 1 << (16 + i)) != 0 {
                        pin[p] = DIRECT[i];
                    }
                }
            }
        }
        if self.king_e != 0 {
            //敵玉が盤面にある時のみ有効

            for i in 0..8 {
                let p = self.search(self.king_e, -DIRECT[i]);
                if (self.ban[p] != KomaInf::Wall) && self.is_e(p) {
                    //敵の駒が有る
                    if (self.control_s[p] & 1 << (16 + i)) != 0 {
                        pin[p] = DIRECT[i];
                    }
                }
            }
        }
    }

    /// 駒の動きとして正しい動きを全て生成する。
    ///
    /// # Return
    ///
    /// * `usize` - 手目。
    pub fn make_legal_moves(
        &mut self,
        s_or_e: KomaInf,
        te_buf: &mut [Te; TE_LEN],
        pin: &mut Option<Pin>, /* =NULL */
    ) -> usize {
        let mut te_num = 0;
        let pin = if let None = pin {
            let mut pbuf: [isize; BAN_LEN] = [0; BAN_LEN];
            self.make_pin_inf(&mut pbuf);
            pbuf
        } else {
            pin.unwrap()
        };

        if s_or_e == KomaInf::Self_ && self.is_control_e(self.king_s) {
            return self.anti_check(s_or_e, te_buf, &pin, self.control_e[self.king_s]);
        }
        if s_or_e == KomaInf::Enemy && self.is_control_s(self.king_e) {
            return self.anti_check(s_or_e, te_buf, &pin, self.control_s[self.king_e]);
        }

        // let suji: isize;
        // let dan: isize;
        let mut start_dan: usize;
        let mut end_dan: usize;
        // 盤上の駒を動かす
        for suji in (0x10..=0x90).step_by(0x10) {
            for dan in 1..=9 {
                if self.is_exists_s_or_e(suji + dan, s_or_e) {
                    self.add_moves(
                        s_or_e,
                        &mut te_num,
                        te_buf,
                        (suji + dan) as USquare,
                        pin[suji + dan],
                        0, // rpin
                    );
                }
            }
        }
        // 歩を打つ
        if self.hand[(s_or_e | KomaInf::FU) as usize] > 0 {
            for suji in (0x10..=0x90).step_by(0x10) {
                // 二歩チェック
                let mut nifu = false;
                for dan in 1..=9 {
                    if self.ban[suji + dan] == s_or_e | KomaInf::FU {
                        nifu = true;
                        break;
                    }
                }
                if nifu {
                    continue;
                };
                //(先手なら２段目より下に、後手なら８段目より上に打つ）
                if s_or_e == KomaInf::Self_ {
                    start_dan = 2;
                    end_dan = 9;
                } else {
                    start_dan = 1;
                    end_dan = 8;
                }
                for dan in start_dan..=end_dan {
                    // 打ち歩詰めもチェック
                    if !self.is_exists(dan as usize + suji)
                        && !self.utifudume(s_or_e, (dan as usize + suji) as USquare, &pin)
                    {
                        te_buf[te_num as usize] = Te::from_4(
                            0,
                            (suji + dan as usize) as USquare,
                            s_or_e | KomaInf::FU,
                            KomaInf::EMP,
                        );
                        te_num += 1;
                    }
                }
            }
        }
        // 香を打つ
        if self.hand[(s_or_e | KomaInf::KY) as usize] > 0 {
            for suji in (0x10..=0x90).step_by(0x10) {
                //(先手なら２段目より下に、後手なら８段目より上に打つ）
                if s_or_e == KomaInf::Self_ {
                    start_dan = 2;
                    end_dan = 9;
                } else {
                    start_dan = 1;
                    end_dan = 8;
                }
                for dan in start_dan..=end_dan {
                    if !self.is_exists(dan as usize + suji) {
                        te_buf[te_num as usize] = Te::from_4(
                            0,
                            (suji + dan as usize) as USquare,
                            s_or_e | KomaInf::KY,
                            KomaInf::EMP,
                        );
                        te_num += 1;
                    }
                }
            }
        }
        //桂を打つ
        if self.hand[(s_or_e | KomaInf::KE) as usize] > 0 {
            //(先手なら３段目より下に、後手なら７段目より上に打つ）
            for suji in (0x10..=0x90).step_by(0x10) {
                if s_or_e == KomaInf::Self_ {
                    start_dan = 3;
                    end_dan = 9;
                } else {
                    start_dan = 1;
                    end_dan = 7;
                }
                for dan in start_dan..=end_dan {
                    if !self.is_exists(dan as usize + suji) {
                        te_buf[te_num as usize] = Te::from_4(
                            0,
                            (suji + dan as usize) as USquare,
                            s_or_e | KomaInf::KE,
                            KomaInf::EMP,
                        );
                        te_num += 1;
                    }
                }
            }
        }
        // 銀～飛車は、どこにでも打てる
        for koma in KomaInf::GI as isize..=KomaInf::HI as isize {
            if let Some(koma_inf) = KomaInf::from_isize(koma) {
                if self.hand[(s_or_e | koma_inf) as usize] > 0 {
                    for suji in (0x10..=0x90).step_by(0x10) {
                        for dan in 1..=9 {
                            if !self.is_exists(dan + suji) {
                                te_buf[te_num] = Te::from_4(
                                    0,
                                    (suji + dan) as USquare,
                                    KomaInf::from_isize(s_or_e as isize | koma).unwrap(),
                                    KomaInf::EMP,
                                );
                                te_num += 1;
                            }
                        }
                    }
                }
            }
        }

        return te_num;
    }

    /// TODO 盤面のfromにある駒を動かす手を生成する。
    fn add_moves(
        &self,
        s_or_e: KomaInf,
        te_num: &mut TeNum,
        te_top: &mut [Te; TE_LEN],
        from: USquare,
        pin: ISquare,
        r_pin: ISquare, /* =0 */
    ) {
        use crate::KomaInf::*;
        match self.ban[from] {
            SFU => self.add_move(s_or_e, te_num, te_top, from, -1, pin, r_pin),
            EFU => self.add_move(s_or_e, te_num, te_top, from, 1, pin, r_pin),
            SKY => self.add_straight(s_or_e, te_num, te_top, from, -1, pin, r_pin),
            EKY => self.add_straight(s_or_e, te_num, te_top, from, 1, pin, r_pin),
            SKE => {
                self.add_move(s_or_e, te_num, te_top, from, 14, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, -18, pin, r_pin);
            }
            EKE => {
                self.add_move(s_or_e, te_num, te_top, from, -14, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, 18, pin, r_pin);
            }
            SGI => {
                self.add_move(s_or_e, te_num, te_top, from, -1, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, -17, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, 15, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, 17, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, -15, pin, r_pin);
            }
            EGI => {
                self.add_move(s_or_e, te_num, te_top, from, 1, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, 17, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, -15, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, -17, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, 15, pin, r_pin);
            }
            SKI | STO | SNY | SNK | SNG => {
                self.add_move(s_or_e, te_num, te_top, from, -1, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, -17, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, 15, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, 1, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, -16, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, 16, pin, r_pin);
            }
            EKI | ETO | ENY | ENK | ENG => {
                self.add_move(s_or_e, te_num, te_top, from, 1, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, 17, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, -15, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, -1, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, -16, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, 16, pin, r_pin);
            }
            SRY | ERY => {
                self.add_move(s_or_e, te_num, te_top, from, 17, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, -15, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, -17, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, 15, pin, r_pin);
            }
            SHI | EHI => {
                self.add_straight(s_or_e, te_num, te_top, from, 1, pin, r_pin);
                self.add_straight(s_or_e, te_num, te_top, from, -1, pin, r_pin);
                self.add_straight(s_or_e, te_num, te_top, from, -16, pin, r_pin);
                self.add_straight(s_or_e, te_num, te_top, from, 16, pin, r_pin);
            }
            SUM | EUM => {
                self.add_move(s_or_e, te_num, te_top, from, 1, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, 16, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, -16, pin, r_pin);
                self.add_move(s_or_e, te_num, te_top, from, -1, pin, r_pin);
            }
            SKA | EKA => {
                self.add_straight(s_or_e, te_num, te_top, from, 17, pin, r_pin);
                self.add_straight(s_or_e, te_num, te_top, from, -17, pin, r_pin);
                self.add_straight(s_or_e, te_num, te_top, from, 15, pin, r_pin);
                self.add_straight(s_or_e, te_num, te_top, from, -15, pin, r_pin);
            }
            SOU | EOU => {
                self.move_king(s_or_e, te_num, te_top, 0); // 王手がかかっている時には、AntiCheckの方が呼ばれるから、Kikiは０です。
            }
            _ => {
                panic!("Unimplemented Koma. {:?}", self.ban[from]);
            }
        }
    }

    /// TODO ある場所の利き情報を作成して返す。普段は使わない関数（差分計算しているから）だが、
    /// 打ち歩詰めのチェックなど、駒を仮に置いてみて何かするようなときに使用する。
    fn count_control_s(&self, sq: ISquare) -> Kiki {
        let mut ret: Kiki = 0;
        let mut dir = 0;
        let mut b = 1;
        let mut bj = 1 << 16;

        while dir < 12 {
            if self.can_move(dir, sq) && self.is_s((sq - DIRECT[dir]) as usize) {
                ret |= b;
            } else if self.can_jump(dir, self.search(sq as USquare, -DIRECT[dir])) {
                let sq2 = self.search(sq as USquare, -DIRECT[dir]) as usize;
                if self.is_s(sq2) {
                    ret |= bj;
                }
            }
            dir += 1;
            b <<= 1;
            bj <<= 1;
        }
        return ret;
    }

    /// TODO
    fn count_control_e(&self, sq: ISquare) -> Kiki {
        let mut ret: Kiki = 0;
        let mut dir = 0;
        let mut b = 1;
        let mut bj = 1 << 16;

        while dir < 12 {
            if self.can_move(dir, sq) && self.is_e((sq - DIRECT[dir]) as USquare) {
                ret |= b;
            } else {
                let sq2 = self.search(sq as USquare, -DIRECT[dir]);
                if self.can_jump(dir, sq2) && self.is_e(sq as USquare) {
                    ret |= bj;
                }
            }
            dir += 1;
            b <<= 1;
            bj <<= 1;
        }
        return ret;
    }

    /// TODO ある場所に移動できる駒を全部集めて、Kiki情報にして返す。
    /// このとき、pinされている駒はpinの方向にしか動けない。
    fn count_move(&self, s_or_e: KomaInf, sq: ISquare, pin: &Pin) -> Kiki {
        let mut ret: Kiki = 0;
        let mut dir = 0;
        let mut b = 1;
        let mut bj = 1 << 16;

        while dir < 12 {
            if self.can_move(dir, sq)
                && self.is_e((sq - DIRECT[dir]) as USquare)
                && (pin[(sq - DIRECT[dir]) as usize] == 0
                    || pin[(sq - DIRECT[dir]) as usize] == DIRECT[dir]
                    || pin[(sq - DIRECT[dir]) as usize] == -DIRECT[dir])
            {
                ret |= b;
            } else {
                let sq2 = self.search(sq as USquare, -DIRECT[dir]);
                if self.can_jump(dir, sq2) {
                    let sq3 = self.search(sq as USquare, -DIRECT[dir]);
                    if self.is_e(sq3) {
                        if pin[self.search(sq as USquare, -DIRECT[dir])] == 0
                            || pin[self.search(sq as USquare, -DIRECT[dir])] == DIRECT[dir]
                            || pin[self.search(sq as USquare, -DIRECT[dir])] == -DIRECT[dir]
                        {
                            ret |= bj;
                        }
                    }
                }
            }
            dir += 1;
            b <<= 1;
            bj <<= 1;
        }
        return ret;
    }

    /// 打ち歩詰めの判定
    fn utifudume(&mut self, s_or_e: KomaInf, to: USquare, pin: &Pin) -> bool {
        if s_or_e == KomaInf::Self_ {
            // まず、玉の頭に歩を打つ手じゃなければ打ち歩詰めの心配はない。
            if self.king_e + 1 != to {
                return false;
            }
        } else {
            // まず、玉の頭に歩を打つ手じゃなければ打ち歩詰めの心配はない。
            if self.king_s - 1 != to {
                return false;
            }
        }
        //実際に歩を打って確かめてみる。
        self.ban[to] = KomaInf::FU | s_or_e;
        if s_or_e == KomaInf::Self_ {
            // 自分の利きがあったら相手は玉で取れない　＆　取る動きを列挙してみたら玉で取る手しかない
            if self.is_control_s(to)
                && (self.count_move(KomaInf::Enemy, to as ISquare, pin) == 1 << 2)
            {
                // 玉に逃げ道があるかどうかをチェック
                for i in 0..8 {
                    if !self.is_e(Kyokumen::get_offset_sq(self.king_e, i))
                        && !self.is_control_s(Kyokumen::get_offset_sq(self.king_e, i))
                    {
                        // 逃げ道があったので、盤面を元の状態に戻して、
                        self.ban[to] = KomaInf::EMP;
                        // 打ち歩詰めではなかった。
                        return false;
                    }
                }
                // 玉の逃げ道もないのなら、打ち歩詰め。盤面の状態は元に戻す。
                self.ban[to] = KomaInf::EMP;
                return true;
            }
            // 玉以外で取る手があるので打ち歩詰めではない。
            self.ban[to] = KomaInf::EMP;
            return false;
        } else {
            // 自分の利きがあったら相手は玉で取れない　＆　取る動きを列挙してみたら玉で取る手しかない
            if self.is_control_e(to)
                && (self.count_move(KomaInf::Self_, to as ISquare, pin) == 1 << 6)
            {
                // 玉に逃げ道があるかどうかをチェック
                for i in 0..8 {
                    if !self.is_s(Kyokumen::get_offset_sq(self.king_s, i))
                        && !self.is_control_e(Kyokumen::get_offset_sq(self.king_s, i))
                    {
                        // 逃げ道があったので、盤面を元の状態に戻して、
                        self.ban[to] = KomaInf::EMP;
                        // 打ち歩詰めではなかった。
                        return false;
                    }
                }
                // 玉の逃げ道もないのなら、打ち歩詰め。盤面の状態は元に戻す。
                self.ban[to] = KomaInf::EMP;
                return true;
            }
            // 玉以外で取る手があるので打ち歩詰めではない。
            self.ban[to] = KomaInf::EMP;
            return false;
        }
    }

    /// TODO ある場所（to）に駒を打つ手の生成
    fn put_to(
        &mut self,
        s_or_e: KomaInf,
        te_num: &mut TeNum,
        te_top: &mut [Te; TE_LEN],
        to: USquare,
        pin: &Pin,
    ) {
        let mut dan: usize = to & 0x0f;
        if s_or_e == KomaInf::Enemy {
            dan = 10 - dan;
        }
        if self.hand[(s_or_e | KomaInf::FU) as usize] > 0 && dan > 1 {
            // 歩を打つ手を生成
            // 二歩チェック
            let suji: usize = to & 0xf0;
            let mut nifu = false;
            for d in 1..=9 {
                if self.ban[suji + d] == (s_or_e | KomaInf::FU) {
                    nifu = true;
                    break;
                }
            }
            // 打ち歩詰めもチェック
            if !nifu && !self.utifudume(s_or_e, to, pin) {
                te_top[*te_num as usize] = Te::from_4(0, to, s_or_e | KomaInf::FU, KomaInf::EMP);
                *te_num += 1;
            }
        }
        if self.hand[(s_or_e | KomaInf::KY) as usize] > 0 && dan > 1 {
            // 香を打つ手を生成
            te_top[*te_num as usize] = Te::from_4(0, to, s_or_e | KomaInf::KY, KomaInf::EMP);
            *te_num += 1;
        }
        if self.hand[(s_or_e | KomaInf::KE) as usize] > 0 && dan > 2 {
            te_top[*te_num as usize] = Te::from_4(0, to, s_or_e | KomaInf::KE, KomaInf::EMP);
            *te_num += 1;
        }
        for koma in KomaInf::GI as usize..=KomaInf::HI as usize {
            if self.hand[s_or_e as usize | koma] > 0 {
                te_top[*te_num as usize] = Te::from_4(
                    0,
                    to,
                    KomaInf::from_usize(s_or_e as usize | koma).unwrap(),
                    KomaInf::EMP,
                );
                *te_num += 1;
            }
        }
    }

    pub fn search(&self, mut sq: USquare, dir: ISquare) -> USquare {
        while {
            sq = (sq as ISquare + dir) as USquare;
            !self.is_exists(sq)
        } {}
        return sq;
    }

    /// TODO 王手を受ける手の生成
    ///
    /// # Return
    ///
    /// * `usize` - 手目。
    pub fn anti_check(
        &mut self,
        s_or_e: KomaInf,
        te_buf: &mut [Te; TE_LEN],
        pin: &Pin,
        kiki: Kiki,
    ) -> usize {
        let king: USquare;
        let mut te_num: TeNum = 0;
        if (kiki & (kiki - 1)) != 0 {
            //両王手は玉を動かすしかない
            self.move_king(s_or_e, &mut te_num, te_buf, kiki);
        } else {
            if s_or_e == KomaInf::Self_ {
                king = self.king_s;
            } else {
                king = self.king_e;
            }
            let check: USquare;
            let mut id: usize = 0;
            while id <= 31 {
                if kiki == (1usize << id) {
                    break;
                }
                id += 1;
            }
            if id < 16 {
                check = Kyokumen::get_offset_sq(king, DIRECT[id]);
            } else {
                check = self.search(king, -DIRECT[id - 16]);
            }
            //王手駒を取る
            self.move_to(s_or_e, &mut te_num, te_buf, check, pin);

            //玉を動かす
            self.move_king(s_or_e, &mut te_num, te_buf, kiki);

            if id >= 16 {
                //合駒をする手を生成する
                let mut i: USquare = Kyokumen::get_offset_sq(king, id as ISquare - 16);
                while !self.is_exists(i) {
                    self.move_to(s_or_e, &mut te_num, te_buf, i, pin); //移動合
                    i = Kyokumen::get_offset_sq(i, id as ISquare - 16);
                }
                let mut i: USquare = Kyokumen::get_offset_sq(king, id as ISquare - 16);
                while !self.is_exists(i) {
                    self.put_to(s_or_e, &mut te_num, te_buf, i, pin); //駒を打つ合
                    i = Kyokumen::get_offset_sq(i, id as ISquare - 16);
                }
            }
        }
        return te_num;
    }

    /// TODO 玉の動く手の生成
    fn move_king(
        &self,
        s_or_e: KomaInf,
        te_num: &mut TeNum,
        te_top: &mut [Te; TE_LEN],
        kiki: Kiki,
    ) {
        let mut id: Option<USquare> = None; //隣接王手駒の位置のid

        // 両王手でないなら王手駒の位置を探す
        for i in 0..8 {
            if (kiki & (1 << i)) != 0 {
                id = Some(i);
                break;
            }
        }
        if let Some(id) = id {
            // 隣接の王手 最初に取る手を生成するのだ
            if s_or_e == KomaInf::Self_ {
                let koma: KomaInf = self.ban[Kyokumen::get_offset_sq(self.king_s, id as isize)];
                if ( koma==KomaInf::EMP || (koma & KomaInf::Enemy).stood())
                    && !self.is_control_e(Kyokumen::get_offset_sq(self.king_s,id as isize)) //敵の駒が効いていない
                    && !(kiki & (1 << (23-id))).stood()
                //敵の飛駒で貫かれていない
                {
                    self.add_move(s_or_e, te_num, te_top, self.king_s, -DIRECT[id], 0, 0);
                }
            } else {
                let koma: KomaInf = self.ban[Kyokumen::get_offset_sq(self.king_e, id as isize)];
                if ( koma==KomaInf::EMP || (koma & KomaInf::Self_).stood())
                    && !self.is_control_s(Kyokumen::get_offset_sq(self.king_e, id as isize)) //敵の駒が効いていない
                    && !(kiki & (1 << (23-id))).stood()
                //敵の飛駒で貫かれていない
                {
                    self.add_move(s_or_e, te_num, te_top, self.king_e, -DIRECT[id], 0, 0);
                }
            }
        }

        for i in 0..8 {
            if Some(i) == id {
                continue;
            }
            if s_or_e == KomaInf::Self_ {
                let koma: KomaInf =
                    self.ban[(self.king_s as ISquare - DIRECT[i as usize]) as usize];
                if ( koma==KomaInf::EMP || (koma & KomaInf::Enemy).stood())
                    && !self.is_control_e(Kyokumen::get_offset_sq(self.king_s, i as isize)) //敵の駒が効いていない
                    && !(kiki & (1 << (23-i))).stood()
                //敵の飛駒で貫かれていない
                {
                    self.add_move(s_or_e, te_num, te_top, self.king_s, -DIRECT[i], 0, 0);
                }
            } else {
                let koma: KomaInf =
                    self.ban[(self.king_e as ISquare - DIRECT[i as usize]) as usize];
                if ( koma==KomaInf::EMP || (koma & KomaInf::Self_).stood())
                        && !self.is_control_s(Kyokumen::get_offset_sq(self.king_e, i as isize)) //敵の駒が効いていない
                        && !(kiki & (1 << (23-i))).stood()
                //敵の飛駒で貫かれていない
                {
                    self.add_move(s_or_e, te_num, te_top, self.king_e, -DIRECT[i], 0, 0);
                }
            }
        }
    }

    /// 手の生成：成り・不成りも意識して、駒の動く手を生成する。
    fn add_move(
        &self,
        s_or_e: KomaInf,
        te_num: &mut TeNum,
        te_top: &mut [Te; TE_LEN],
        from: USquare,
        diff: ISquare,
        pin: ISquare,
        r_pin: ISquare, /* =0 */
    ) {
        if r_pin == diff || r_pin == -diff {
            return;
        }
        let to: USquare = (from as ISquare + diff) as usize;
        let dan: usize = to & 0x0f;
        let from_dan: usize = from & 0x0f;

        if (pin == 0 || pin == diff || pin == -diff) && !(self.ban[to] & s_or_e).stood() {
            if self.ban[from] == KomaInf::SKE && dan <= 2 {
                // 必ず成る
                te_top[*te_num as usize] = Te::from_5(from, to, self.ban[from], self.ban[to], 1);
                *te_num += 1;
            } else if (self.ban[from] == KomaInf::SFU || self.ban[from] == KomaInf::SKY) && dan <= 1
            {
                // 必ず成る
                te_top[*te_num as usize] = Te::from_5(from, to, self.ban[from], self.ban[to], 1);
                *te_num += 1;
            } else if self.ban[from] == KomaInf::EKE && dan >= 8 {
                // 必ず成る
                te_top[*te_num as usize] = Te::from_5(from, to, self.ban[from], self.ban[to], 1);
                *te_num += 1;
            } else if (self.ban[from] == KomaInf::EFU || self.ban[from] == KomaInf::EKY) && dan >= 9
            {
                // 必ず成る
                te_top[*te_num as usize] = Te::from_5(from, to, self.ban[from], self.ban[to], 1);
                *te_num += 1;
            } else {
                if s_or_e == KomaInf::Self_ && (from_dan <= 3 || dan <= 3) && self.can_promote(from)
                {
                    te_top[*te_num as usize] =
                        Te::from_5(from, to, self.ban[from], self.ban[to], 1);
                    *te_num += 1;
                } else if s_or_e == KomaInf::Enemy
                    && (from_dan >= 7 || dan >= 7)
                    && self.can_promote(from)
                {
                    te_top[*te_num as usize] =
                        Te::from_5(from, to, self.ban[from], self.ban[to], 1);
                    *te_num += 1;
                }
                // 成らない手も生成する。
                te_top[*te_num as usize] = Te::from_5(from, to, self.ban[from], self.ban[to], 0);
                *te_num += 1;
            }
        }
    }

    /// 飛車角香車がまっすぐに進む手の生成
    fn add_straight(
        &self,
        s_or_e: KomaInf,
        te_num: &mut TeNum,
        te_top: &mut [Te; TE_LEN],
        from: USquare,
        dir: ISquare,
        pin: ISquare,
        r_pin: ISquare, /* =0 */
    ) {
        if dir == r_pin || dir == -r_pin {
            return;
        }
        let mut i: isize;
        if pin == 0 || pin == dir || pin == -dir {
            // 空白の間、動く手を生成する
            i = dir;
            while self.ban[(from as isize + i) as usize] == KomaInf::EMP {
                self.add_move(s_or_e, te_num, te_top, from, i, 0, 0);
                i += dir;
            }
            // 味方の駒でないなら、そこへ動く
            if !((self.ban[(from as isize + i) as usize] & s_or_e).stood()) {
                self.add_move(s_or_e, te_num, te_top, from, i, 0, 0);
            }
        }
    }

    /// toに動く手の生成
    fn move_to(
        &mut self,
        s_or_e: KomaInf,
        te_num: &mut TeNum,
        te_top: &mut [Te; TE_LEN],
        to: USquare,
        pin: &Pin,
    ) {
        let mut dan: USquare = to & 0x0f;
        if s_or_e == KomaInf::Enemy {
            dan = 10 - dan;
        }
        if self.hand[(s_or_e | KomaInf::FU) as usize] > 0 && dan > 1 {
            // 歩を打つ手を生成
            // 二歩チェック
            let suji: USquare = to & 0xf0;
            let mut nifu: bool = false;
            for d in 1..=9 {
                if self.ban[suji + d] == s_or_e | KomaInf::FU {
                    nifu = true;
                    break;
                }
            }
            // 打ち歩詰めもチェック
            if !nifu && !self.utifudume(s_or_e, to, pin) {
                te_top[*te_num as usize] = Te::from_4(0, to, s_or_e | KomaInf::FU, KomaInf::EMP);
                *te_num += 1;
            }
        }
        if self.hand[(s_or_e | KomaInf::KY) as usize] > 0 && dan > 1 {
            // 香を打つ手を生成
            te_top[*te_num as usize] = Te::from_4(0, to, s_or_e | KomaInf::KY, KomaInf::EMP);
            *te_num += 1;
        }
        if self.hand[(s_or_e | KomaInf::KE) as usize] > 0 && dan > 2 {
            te_top[*te_num as usize] = Te::from_4(0, to, s_or_e | KomaInf::KE, KomaInf::EMP);
            *te_num += 1;
        }
        for koma in KomaInf::GI as usize..=KomaInf::HI as usize {
            if self.hand[s_or_e as usize | koma as usize] > 0 {
                te_top[*te_num as usize] = Te::from_4(
                    0,
                    to,
                    KomaInf::from_usize(s_or_e as usize | koma as usize).unwrap(),
                    KomaInf::EMP,
                );
                *te_num += 1;
            }
        }
    }

    /// それっぽく表示する。
    pub fn print(&self) {
        const NUM_STR9: [&str; 9] = ["一", "二", "三", "四", "五", "六", "七", "八", "九"];
        const NUM_STR18: [&str; 18] = [
            "一", "二", "三", "四", "五", "六", "七", "八", "九", "10", "11", "12", "13", "14",
            "15", "16", "17", "18",
        ];
        let mut x;
        let mut y = 0;
        print!("持ち駒：");
        x = KomaInf::EHI as usize;
        while x >= KomaInf::EFU as usize {
            if self.hand[x] > 1 {
                y = 1;
                print!("{}{}", KOMA_STR2[x], NUM_STR18[self.hand[x] - 1]);
            } else if self.hand[x] == 1 {
                y = 1;
                print!("{}", KOMA_STR2[x]);
            }
            x -= 1;
        }
        if y != 0 {
            print!("\n");
        } else {
            print!("なし\n");
        }
        print!("  ９ ８ ７ ６ ５ ４ ３ ２ １ \n");
        print!("+---------------------------+\n");
        for y in 1..=9 {
            print!("|");
            x = 9;
            while x >= 1 {
                print!("{}", KOMA_STR[self.ban[x * 16 + y] as usize]);
                x -= 1;
            }
            print!("|{}", NUM_STR9[y - 1]);
            print!("\n");
        }
        print!("+---------------------------+\n");
        print!("持ち駒：");
        y = 0;
        x = KomaInf::SHI as usize;
        while x >= KomaInf::SFU as usize {
            if self.hand[x] > 1 {
                y = 1;
                print!("{}{}", KOMA_STR2[x], NUM_STR18[self.hand[x] - 1]);
            } else if self.hand[x] == 1 {
                y = 1;
                print!("{}", KOMA_STR2[x]);
            }
            x -= 1;
        }
        if y != 0 {
            print!("\n");
        } else {
            print!("なし\n");
        }
    }

    /* TODO
    pub fn print() {
        FPrint(stdout);
    }
    */
}
