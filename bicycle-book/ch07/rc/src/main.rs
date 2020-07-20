use std::rc::Rc;

#[derive(Debug)]
struct Child(i32);

fn main() {
    let mut rc1 = Rc::new(Child(1)); // Rcポインタ経由でChild値をヒープ領域に格納する

    // strong_countでこのChild値の参照カウント（共同所有者の数）が得られる
    println!("(a) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);
}
