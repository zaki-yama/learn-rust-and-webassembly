fn hello() {
    println!("Hello");
}

fn main() {
    // --- 4-2-1 ユニット
    let ret = hello();
    assert_eq!(ret, ());
    // サイズは0バイト
    assert_eq!(std::mem::size_of::<()>(), 0);

    // --- 4-2-2 真理値
    let b1 = true;
    let b2 = !b1;
    assert_eq!(b2, false);

    // サイズは1バイト
    assert_eq!(std::mem::size_of::<bool>(), 1);

    // --- 4-2-3 固定精度整数
    let n1 = 10_000; // デフォルトで i32 型
    let n2 = 0_u8; // u8 型

    // ^ はべき乗でなく排他的論理和(XOR)
    // べき乗の場合は pow() を使う
    let pow = 2_usize.pow(32);

    let n1 = std::u8::MAX; // 255u8
    let n2 = 1_u8;
    // 答えは256だがu8型では表現できない（オーバーフロー）
    // let n3 = n1 + n2;
    // println!("{}", n3);

    // オーバーフローを想定したメソッドのうち、ラッピング演算(wrapping_xxx)
    // 200 * 3 = 600 だが u8 の最大値は256
    // 計算結果は600を256で割った余りの88になる
    assert_eq!(200u8.wrapping_mul(3u8), 88);
}
