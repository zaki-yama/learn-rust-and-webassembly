struct Foo {
    value: i32,
}

fn main() {
    let f = Foo { value: 42 };
    let ref_ref_ref_f = &&&f;
    println!("{}", ref_ref_ref_f.value);
}
