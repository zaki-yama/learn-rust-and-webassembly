/// 参照による所有権の可変な借用

struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f はここでドロップ
}

fn main() {
    let mut foo = Foo { x: 42 };
    let f = &mut foo;

    // 失敗: do_something(foo) はここでエラー
    // foo は可変に借用されており移動できないため

    // 失敗: foo.x = 13; はここでエラー
    // foo は可変に借用されている間は変更できないため

    f.x = 13;
    // f はここから先では使用されないため、ここでドロップ

    println!("{}", foo.x);

    // 可変な借用はドロップされているため変更可能
    foo.x = 7;

    // foo の所有権を関数に移動
    do_something(foo);
}
