pub mod koma_inf;

use koma_inf::KomaInf;

fn main() {
    println!("Kifuwarabe's shogi with Usapyon");

    let banpadding: [KomaInf; 16] = [KomaInf::EMP; 16];
    let ban: [KomaInf; 16 * (9 + 2)] = [KomaInf::EMP; 16 * (9 + 2)];
}
