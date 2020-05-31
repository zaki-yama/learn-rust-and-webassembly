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

    // --- 4-2-6 参照
    let mut n = 0;
    println!("main:     n = {}", n);

    f1(n);
    println!("main:     n = {}", n);

    // `&mut n`でnの値を指す可変のポインタを作成する
    f2(&mut n);
    println!("main:     n = {}", n);
}

// 関数f1は呼び出し元の値のコピーを引数nに束縛し、1に変更する
fn f1(mut n: u32) {
    n = 1;
    println!("f1:       n = {}", n);
}

// 関数f2は呼び出し元の値を指すポインタを受け取り、
// ポインタが指す場所に2を格納する
fn f2(n_ptr: &mut u32) {
    println!("f2        n_ptr = {:p}", n_ptr);

    // *をつけると参照先にアクセスできる。これを参照外し(dereference)と呼ぶ
    *n_ptr = 2;
    println!("f2:       *n_ptr = {}", *n_ptr);
}
