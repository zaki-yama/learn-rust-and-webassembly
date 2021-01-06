// 可変な参照
// https://doc.rust-jp.rs/book-ja/ch04-02-references-and-borrowing.html#%E5%8F%AF%E5%A4%89%E3%81%AA%E5%8F%82%E7%85%A7

fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("r1 = {}", r1);
    println!("r2 = {}", r2);
    change(&mut s);
    println!("s = {}", s);
}

fn change(some_string: &mut String) {
    // cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    some_string.push_str(", world");
}
