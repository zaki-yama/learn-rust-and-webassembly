struct Foo {
    x: i32,
}

// 引数 foo と戻り値はライフタイムを共有
fn do_something<'a>(foo: &'a Foo) -> &'a i32 {
    return &foo.x;
}

fn main() {
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // x はここでドロップされるため、不変な参照が作成可能
    let y = do_something(&foo);
    println!("{}", y);
    // y はここでドロップ
    // foo はここでドロップ
}
