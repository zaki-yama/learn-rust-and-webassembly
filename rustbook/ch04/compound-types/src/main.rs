fn main() {
    // --- 4-3-1 タプル
    let t1 = (88, true);

    assert_eq!(t1.0, 88);
    assert_eq!(t1.1, true);

    let i = 0;
    // let t1a = t1.i; // error

    let mut t1 = (88, true);
    t1.0 += 100;
    assert_eq!(t1, (188, true));

    // パターンマッチで分解する
    let (n1, b1) = (88, true);
    assert_eq!(n1, 88);
    assert_eq!(b1, true);

    let ((x1, y1), (x2, y2)) = ((0, 5), (10, -1));
    assert_eq!(x1, 0);
    assert_eq!(y1, 5);
    assert_eq!(x2, 10);
    assert_eq!(y2, -1);
}
