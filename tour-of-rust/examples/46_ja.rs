struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f はここでドロップ
}

fn main() {
    let foo = Foo { x: 42 };
    // foo の所有権は do_something に移動
    do_something(foo);

    // foo は使えなくなる
    // borrow of moved value: `foo`
    // value borrowed here after move
    // println!("{}", foo.x);
}
