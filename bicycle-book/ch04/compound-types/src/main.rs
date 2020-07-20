// 4-3 プリミティブな複合型
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
    println!();

    // --- 4-3-3 スライス
    let a1 = ['a', 'b', 'c', 'd'];
    println!("a1: {:?}", a1);

    print_info("&a1[..]", &a1[..]); // 全要素のスライス
    print_info("&a1", &a1); // 同上
    print_info("&a1[1..3]", &a1[1..3]);

    // ベクタ
    let v1 = vec!['e', 'f', 'g', 'h'];
    println!("\nv1: {:?}", v1);

    print_info("&v1[..]", &v1[..]);
    print_info("&v1", &v1);
    print_info("&v1[1..3]", &v1[1..3]);

    // ミュータブルなスライス
    let mut a1 = [5, 4, 3, 2];
    let s1 = &mut a1[1..3];
    s1[0] = 6;
    s1[1] *= 10;
    s1.swap(0, 1);
    assert_eq!(s1, [30, 6]);

    assert_eq!(a1, [5, 30, 6, 2]); // スライスを通じて配列の内容が変更された

    let a2: [i32; 0] = [];
    let s2 = &a2;
    assert!(s2.is_empty());
    assert_eq!(s2.len(), 0);
    assert_eq!(s2.first(), None);

    let a3 = ["zero", "one", "two", "three", "four"];
    let s3 = &a3[1..4];
    assert!(!s3.is_empty());
    assert_eq!(s3.len(), 3);
    assert_eq!(s3.first(), Some(&"one"));

    assert_eq!(s3[1], "two");
    // assert_eq!(s3[3], "?"); // panic
    assert_eq!(s3.get(1), Some(&"two"));
    assert_eq!(s3.get(3), None);

    assert!(s3.contains(&"two"));
    assert!(s3.starts_with(&["one", "two"]));
    assert!(s3.ends_with(&["two", "three"]));

    // --- 4-3-4 文字列スライス
    let s1 = "abc1"; // &'static str型
    let s2 = "abc2";
    assert!(s1 < s2);
    assert!(s1 != s2);

    let s6 = r#"文字列に"と\を含める"#; // raw文字列リテラル。正規表現などに便利
    let s7 = r###"このように#の数を増やすと"##"があっても大丈夫"###;
    assert_eq!(s7, "このように#の数を増やすと\"##\"があっても大丈夫");

    let fruits = "あかりんご, あおりんご\nラズベリー, ブラックベリー";
    // lines()メソッドは改行コード(\n)を含む文字列から1行ずつ
    // 取り出せるイテレータを作る
    let mut lines = fruits.lines();
    // イテレータのnext()メソッドで次の行を得る
    let apple_line = lines.next();
    assert_eq!(apple_line, Some("あかりんご, あおりんご"));
    assert_eq!(lines.next(), Some("ラズベリー, ブラックベリー"));
    // 次の行がないならNoneが返る
    assert_eq!(lines.next(), None);

    // りんごの行(Some(..))の中身を取り出す
    if let Some(apples) = apple_line {
        assert!(apples.starts_with("あか"));
        assert!(apples.contains("りんご"));
        // 「あお」が出現する位置はUTF-8表現で何バイト目か
        assert_eq!(apples.find("あお"), Some(17));
    } else {
        unreachable!();
    }

    let s = "かか\u{3099}く"; // \u{3099}は濁点文字
    println!("{}", s);

    let mut iter = s.chars(); // char型のイテレータ
    assert_eq!(iter.next(), Some('か'));
    assert_eq!(iter.next(), Some('か'));
    assert_eq!(iter.next(), Some('\u{3099}'));
    assert_eq!(iter.next(), Some('く'));
    assert_eq!(iter.next(), None);
}

// &[char]型のスライスを引数に取り、その情報を表示する
fn print_info(name: &str, sl: &[char]) {
    println!(
        "  {:9} - {}, {:?}, {:?}, {:?}",
        name,
        sl.len(),   // 長さ(バイト数) usize
        sl.first(), // Option<char>
        sl[1],      // char
        sl.last()   // Option<char>
    );
}
