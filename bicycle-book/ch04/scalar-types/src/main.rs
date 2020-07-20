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

    // --- 4-2-7 生ポインタ
    let c1 = 'A';
    let c1_ptr: *const char = &c1;
    // 生ポインタの参照外しはunsafeな操作
    assert_eq!(unsafe { *c1_ptr }, 'A');

    let mut n1 = 0;
    let n1_ptr: *mut i32 = &mut n1;
    assert_eq!(unsafe { *n1_ptr }, 0);

    // 可変の生ポインタでは参照先の値を変更できる
    unsafe {
        *n1_ptr = 1_000;
        assert_eq!(*n1_ptr, 1_000);
    }

    // --- 4-2-8 関数ポインタ
    // 変数に型注釈として関数ポインタ型を指定することで、関数名から関数ポインタを得られる
    let mut f: fn(i32) -> i32 = double;
    assert_eq!(f(-42), -84);

    f = abs;
    assert_eq!(f(-42), 42);

    // 関数ポインタのサイズはusizeと同じ
    assert_eq!(std::mem::size_of_val(&f), std::mem::size_of::<usize>());

    // 変数に型注釈を付けないと関数定義型だと推論される
    let mut f_bad = double;
    // 関数定義型は関数ごとに異なる型になるので、変数f_badに別の関数定義型を束縛できない
    // f_bad = abs;
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

fn double(n: i32) -> i32 {
    n + n
}

fn abs(n: i32) -> i32 {
    if n >= 0 {
        n
    } else {
        -n
    }
}
