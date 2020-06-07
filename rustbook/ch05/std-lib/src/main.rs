use std::collections::HashMap;

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

    // --- 5-2-3 その他のコレクション型
    let mut m1 = HashMap::new();

    m1.insert("a", 1);
    m1.insert("b", 3);
    assert_eq!(m1.len(), 2);

    assert_eq!(m1.get("b"), Some(&3));
    assert_eq!(m1.get("c"), None);

    let d = m1.entry("d").or_insert(0);
    *d += 7;
    assert_eq!(m1.get("d"), Some(&7));

    // --- 5-2-4 String
    // strリテラルからStringを作る。どちらの方法でも結果は同じ
    let mut s1 = "ラズベリー".to_string();
    let mut s2 = String::from("ブラックベリー");

    s1.push_str("タルト");
    assert_eq!(s1, "ラズベリータルト");

    s2.push('と');

    // push_str()が受け付けるのは&str型のみ。以下はコンパイルエラーになる
    let s3 = String::from("ストロベリー");
    // s2.push_str(s3); // error

    // &をつけると型強制というしくみによって&Stringから&strへ変換される
    s2.push_str(&s3);
    assert_eq!(s2, "ブラックベリーとストロベリー");

    let i = 42;
    assert_eq!(i.to_string(), "42");

    let f = 4.3 + 0.1;
    assert_eq!(f.to_string(), "4.3999999999999995");
    assert_eq!(format!("{:.2}", f), "4.40");

    let t = (1, "ABC");
    // 2要素のタプル型はDebugトレイトを実装しているのでformat!マクロで変換できる
    assert_eq!(format!("{:?}", t), r#"(1, "ABC")"#);

    let s1 = "42";
    assert_eq!(s1.parse::<i32>(), Ok(42)); // &str型からi32型へ変換

    let s2 = "abc";
    // 変数の型から推論できるならparseの型パラメータは不要
    let r2: Result<f64, _> = s2.parse();
    assert!(r2.is_err());
    println!("{:?}", r2);

    let cs = ['t', 'r', 'u', 's', 't']; // [char; 5]
    assert_eq!(cs.iter().collect::<String>(), "trust");
    assert_eq!(&cs[1..].iter().collect::<String>(), "rust");

    let bad_utf8: [u8; 7] = [
        b'a', // a
        0xf0, 0x90, 0x80, // でたらめなバイト列
        0xe3, 0x81, 0x82, // あ
    ];

    // 不正なバイト列はUnnicodeのU+FFFD Replacement Characterに置き換わる
    let s = String::from_utf8_lossy(&bad_utf8);
    assert_eq!(s, "a\u{fffd}あ");
}

// 間違った戻り値の型
// fn f1(name: &str) -> &str {
//    let s = format!("Hello, {}!", name);
//    Stringから&strを作成し、戻り値として返す
//    // コンパイルエラー
//    &s
// }

// 正しい戻り値の型
fn f1(name: &str) -> String {
    format!("Hello, {}!", name)
}
