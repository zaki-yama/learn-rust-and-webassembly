// https://doc.rust-jp.rs/book-ja/ch04-01-what-is-ownership.html#%E6%88%BB%E3%82%8A%E5%80%A4%E3%81%A8%E3%82%B9%E3%82%B3%E3%83%BC%E3%83%97
fn main() {
    let s1 = gives_ownership();
    println!("s1 = {}", s1);

    let s2 = String::from("hello");
    println!("s2 = {}", s2);

    let s3 = takes_and_gives_back(s2);
    // "borrow of moved value" error
    // println!("s2 = {}", s2);
    println!("s3 = {}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
