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

    // --- 5-2-5 範囲
    let a = ['a', 'b', 'c', 'd', 'e'];

    // 糖衣構文と実際の範囲の対応
    assert_eq!(a[..], ['a', 'b', 'c', 'd', 'e']);
    assert_eq!(a[..3], ['a', 'b', 'c']);
    assert_eq!(a[..=3], ['a', 'b', 'c', 'd']);
    assert_eq!(a[1..], ['b', 'c', 'd', 'e']);
    assert_eq!(a[1..3], ['b', 'c']);
    assert_eq!(a[1..=3], ['b', 'c', 'd']);

    // 糖衣構文とRange*型の対応
    assert_eq!(.., std::ops::RangeFull);
    assert_eq!(..3, std::ops::RangeTo { end: 3 });
    assert_eq!(..=3, std::ops::RangeToInclusive { end: 3 });
    assert_eq!(1.., std::ops::RangeFrom { start: 1 });
    assert_eq!(1..3, std::ops::Range { start: 1, end: 3 });
    assert_eq!(1..=3, std::ops::RangeInclusive::new(1, 3));

    // --- 5-2-6 オプション
    let a1 = ['a', 'b', 'c', 'd'];
    assert_eq!(a1.get(0), Some(&'a'));
    assert_eq!(a1.get(4), None);

    let mut o1 = Some(10);
    match o1 {
        Some(s) => assert_eq!(s, 10), // パターンマッチで中の値を取り出す
        None => unreachable!(),
    }

    o1 = Some(20);
    if let Some(s) = o1 {
        // if let式でもバリアントの判別と値の取り出しができる
        assert_eq!(s, 20);
    }

    let mut o2 = Some(String::from("Hello")); // Option<String>
    assert_eq!(o2.unwrap(), "Hello");

    // unwrap()はNoneのときにpanicするので、できるだけ使わない方がいい
    o2 = None;
    // o2.unwrap(); // panicked

    // unwrap_or_else()ならNoneでもpanicしないので安心して使える
    // Noneのときはクロージャを実行し、Noneの代わりになる値を得る
    assert_eq!(
        o2.unwrap_or_else(|| String::from("o2 is none")),
        "o2 is none"
    );

    // Someで包まれた値を操作するならmap()やand_then()などのコンビネータが便利

    // map()はSome(値)のときは値にクロージャを適用し、クロージャが返した値をSomeで包み直す
    let mut o3 = Some(25);
    assert_eq!(o3.map(|n| n * 10), Some(250));

    // NoneならなにもせずNoneを返す
    o3 = None;
    assert_eq!(o3.map(|n| n * 10), None);

    o3 = Some(10);
    assert_eq!(
        o3.map(|n| n * 10)
            // and_then()はSome(値)のときは値にクロージャを適用し
            // クロージャが返した値(Some(新しい値), またはNone)をそのまま返す
            .and_then(|n| if n >= 200 { Some(n) } else { None }),
        None
    );

    assert_eq!(add_elems(&[3, 7, 31, 127]), Some(3 + 127));
    assert_eq!(add_elems(&[7, 11]), None);

    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);

    // --- 5-2-7 リザルト
    assert_eq!("10".parse::<i32>(), Ok(10)); // 変換できたらOK(値)が返される
    let res0 = "a".parse::<i32>(); // 変換できなかったら`Err(エラーを表す値)`が返される
    assert!(res0.is_err());
    println!("{:?}", res0);

    assert_eq!(add0("3", "127"), Ok(3 + 127));
    assert!(add0("3", "abc").is_err());

    assert_eq!(add1("3", "abc"), Err("s1が整数ではありません".to_string()));
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

// Option
fn add_elems(s: &[i32]) -> Option<i32> {
    // 複数のOption値を扱うときは?演算子が便利
    // Some(値)なら値を取り出し、Noneならこの関数からすぐに戻る(Noneを返す)
    let s0 = s.get(0)?;
    let s3 = s.get(3)?;
    Some(s0 + s3)
}

// and_then()の補足
// ref. https://doc.rust-jp.rs/rust-by-example-ja/error/option_unwrap/and_then.html
#[derive(Debug)]
enum Food {
    CordonBleu,
    Steak,
    Sushi,
}
#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

// 我々は寿司の材料を持っていない
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

// コルドン・ブルー(Cordon Bleu)のレシピも持っていない。
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

// 料理を作るためには、材料とレシピの両方が必要。
// ロジックの流れを`match`のチェインで表す。
fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None => None,
        Some(food) => match have_ingredients(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

// `and_then()`を用いることで、同じことをよりコンパクトに表現できる。
fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

// mapで書く場合、ネストするとOptionがネストする
fn cookable_v1_1(food: Food) -> Option<Option<Food>> {
    have_recipe(food).map(|food| have_ingredients(food))
}

fn eat(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

// 複数のResult値を扱うときは?演算子が便利
fn add0(s0: &str, s1: &str) -> Result<i32, std::num::ParseIntError> {
    let s0 = s0.parse::<i32>()?;
    let s1 = s1.parse::<i32>()?;
    Ok(s0 + s1)
}

fn add1(s0: &str, s1: &str) -> Result<i32, String> {
    let s0 = s0.parse::<i32>().map_err(|_e| "s0が整数ではありません")?;
    let s1 = s1.parse::<i32>().map_err(|_e| "s0が整数ではありません")?;
    Ok(s0 + s1)
}
