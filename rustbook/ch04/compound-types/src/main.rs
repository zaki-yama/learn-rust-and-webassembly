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

    // --- 4-3-2 配列
    let a1 = [false, true, false]; // [bool; 3]型
    let a2 = [0.0, -1.0, 1.0, 0.5]; // [f64; 4]型

    assert_eq!(a2.len(), 4);

    // 長さ100の配列を作り、全要素を0i32で初期化する
    // (要素の型はCopyトレイトを実装していなければならない)
    let a3 = [0; 100];

    let a4 = [['a', 'b'], ['c', 'd']]; // [[char; 2]; 2]型

    // let a5 = [false, 'a']; // error

    // 配列の長さは実行時に指定できない
    let size = 100;
    // let a1 = [0; size]; // error(E0435)

    // ベクタならOK
    let mut v1 = vec![0; size];
    assert_eq!(v1.len(), 100);

    // ベクタは要素の追加・削除が可能
    v1.push(1);
    assert_eq!(v1.len(), 101);
    assert_eq!(v1.pop(), Some(1));
    assert_eq!(v1.len(), 100);

    // 要素へのアクセス1: インデックス
    let array1 = ['H', 'e', 'l', 'l', 'o'];
    assert_eq!(array1[1], 'e');

    let mut array2 = [0, 1, 2];
    array2[1] = 10;

    let mut index = 0;
    assert_eq!(array2[index], 0);
    index += 1;
    assert_eq!(array2[index], 10);

    // インデックスの範囲外にアクセスしようとしているのが
    // コンパイル時に分かる場合はコンパイルエラー、
    // 分からなければ実行時にパニック
    let array3 = [0, 1];
    // array3[2]; // error

    let index = 2;
    // array3[index]; // 実行時にパニック

    // get(): より安全な値の取り出し
    assert_eq!(array3.get(1), Some(&1));
    assert_eq!(array3.get(2), None);

    let array4 = ['a'; 50];
    for ch in array4.iter() {
        print!("{},", *ch);
    }
}
