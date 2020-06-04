fn main() {
    // --- 5-2-1 Box(std::boxed::Box<T>)
    let t1 = (3, "birds".to_string()); // (i32, String)型のタプル。スタックに置かれる
    let mut b1 = Box::new(t1);
    (*b1).0 += 1;
    assert_eq!(*b1, (4, "birds".to_string()));

    // Box::new()の実行後にt1にアクセスしようとするとコンパイルエラーになる
    // println!("{:?}", &t1);
}
