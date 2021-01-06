// 文字列スライス
// https://doc.rust-jp.rs/book-ja/ch04-03-slices.html#%E6%96%87%E5%AD%97%E5%88%97%E3%82%B9%E3%83%A9%E3%82%A4%E3%82%B9

fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let hello = &s[..5];
    let world = &s[6..];
}
