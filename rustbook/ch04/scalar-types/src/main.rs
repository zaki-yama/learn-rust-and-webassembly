fn hello() {
    println!("Hello");
}

fn main() {
    // --- 4-2-1 ユニット
    let ret = hello();
    assert_eq!(ret, ());
    // サイズは0バイト
    assert_eq!(std::mem::size_of::<()>(), 0);
}
