fn main() {
    // 文字をcharのベクトルとして集める
    let chars = "hi 🦀".chars().collect::<Vec<char>>();
    println!("{}", chars.len()); // should be 4
    // chars は 4 バイトなので、u32 に変換することができる
    println!("{}", chars[2] as u32);
}
