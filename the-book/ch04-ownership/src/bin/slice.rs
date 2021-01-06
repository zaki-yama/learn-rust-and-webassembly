fn main() {
    let my_string = String::from("hello world");

    let word = first_word(&my_string);

    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);
    // 文字列リテラルはすでに文字列スライスなので
    // スライス記法なしでも機能するのだ！
    let word = first_word(my_string_literal);
    println!("the first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
