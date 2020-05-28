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
}
