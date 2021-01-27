// 15.1. > Enabling Recursive Types with Boxes
// ボックスで再帰的な型を可能にする

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
