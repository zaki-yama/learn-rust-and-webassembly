[The Rust Programming Language](https://doc.rust-jp.rs/book/second-edition/foreword.html)
===============================

## キーワード
- 式指向言語

### [3.1 変数と可変性 - The Rust Programming Language](https://doc.rust-jp.rs/book/second-edition/ch03-01-variables-and-mutability.html)
- 変数 (`let`) はデフォルトでimmutable。mutableにしたいときは `mut` をつける
- シャドーイング

### [3.2 データ型 - The Rust Programming Language](https://doc.rust-jp.rs/book/second-edition/ch03-02-data-types.html)
- スカラー型
  - 整数型
    - `i8` (符号つき) と `u8` (符号なし)、とかはよくある話
  - 浮動小数点型 (`f32`, `f64`。基本は `f64`)
  - 論理値型 (`bool`)
  - 文字型
- 複合型
  - タプル型
  - 配列型

### [3.3 関数の動作法 - The Rust Programming Language](https://doc.rust-jp.rs/book/second-edition/ch03-03-how-functions-work.html)
- 式と文：式は `;` がつかない

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1; // error
}
```
