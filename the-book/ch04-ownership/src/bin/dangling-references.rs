// 宙に浮いた参照
// https://doc.rust-jp.rs/book-ja/ch04-02-references-and-borrowing.html#%E5%AE%99%E3%81%AB%E6%B5%AE%E3%81%84%E3%81%9F%E5%8F%82%E7%85%A7

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
