fn main() {
    let v: Vec<i32> = Vec::new();
    println!("Hello, world!");
    let v2 = vec![1, 2, 3];

    // 値の追加
    let mut v3 = Vec::new();
    v3.push(5);
    
    // 値の参照
    // 1. [index]でアクセス
    // 2. .get()を使う
    let v4 = vec![1, 2, 3, 4, 5];

    let third = &v4[2];
    println!("The third element is {}", third);

    match v4.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 「可変な参照と不変な参照を組み合わせることはできない」ルールはVectorにも適用される
    // v4.push(6); // error

    // ループで回す
    let v5 = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // 異なる複数の型を扱いたい場合はEnumを使う
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

