fn main() {
    // --- 5-2-1 Box(std::boxed::Box<T>)
    let t1 = (3, "birds".to_string()); // (i32, String)型のタプル。スタックに置かれる
    let mut b1 = Box::new(t1);
    (*b1).0 += 1;
    assert_eq!(*b1, (4, "birds".to_string()));

    // Box::new()の実行後にt1にアクセスしようとするとコンパイルエラーになる
    // println!("{:?}", &t1);

    // --- 5-2-2 ベクタ
    let v1 = vec![false, true, false]; // Vec<bool>
    let v2 = vec![0.0, -1.0, 1.0, 0.5]; // Vec<f64>

    assert_eq!(v2.len(), 4);

    let v3 = vec![0; 100];

    // ベクタは入れ子にできる。子の要素数はそれぞれが異なってもかまわない
    let v4 = vec![vec!['a', 'b', 'c'], vec!['d']]; // Vec<Vec<char>>型

    let mut v6 = vec!['a', 'b', 'c'];
    v6.push('d');
    v6.push('e');
    assert_eq!(v6, ['a', 'b', 'c', 'd', 'e']);

    assert_eq!(v6.pop(), Some('e'));
    v6.insert(1, 'f');
    assert_eq!(v6.remove(2), 'b');
    assert_eq!(v6, ['a', 'f', 'c', 'd']);
}
