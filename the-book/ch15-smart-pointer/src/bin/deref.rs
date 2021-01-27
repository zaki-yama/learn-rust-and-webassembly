use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    // Derefトレイトを実装して型を参照のように扱う
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // *y がコンパイルエラーにならないのはDerefトレイとを実装しているから
    assert_eq!(5, *y);

    // 暗黙的な参照外し型矯正
    let m = MyBox::new(String::from("Rust"));

    // &m は &MyBox<String> 型
    hello(&m);
    // 参照外し型強制がないと、こう書かないといけない
    hello(&(*m)[..]);
}
