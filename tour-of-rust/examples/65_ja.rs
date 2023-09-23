/// 文字列スライス
fn main() {
    let a = "hi 🦀";
    println!("{}", a.len());
    let first_word = &a[0..2];
    let second_word = &a[3..7];
    // let half_crab = &a[3..5]; は失敗します。
    // Rust は無効な unicode 文字のスライスを受け付けません。
    println!("{} {}", first_word, second_word);
}
