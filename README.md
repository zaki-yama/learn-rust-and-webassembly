[The Rust Programming Language](https://doc.rust-jp.rs/book/second-edition/foreword.html)
===============================

## キーワード
- 式指向言語

### [3.1 変数と可変性](https://doc.rust-jp.rs/book/second-edition/ch03-01-variables-and-mutability.html)
- 変数 (`let`) はデフォルトでimmutable。mutableにしたいときは `mut` をつける
- シャドーイング
  - 一度宣言した変数を同名で上書きできる

```rust
let spaces = "    ";
let spaces = spaces.len();
```

### [3.2 データ型](https://doc.rust-jp.rs/book/second-edition/ch03-02-data-types.html)
- スカラー型
  - 整数型
    - `i8` (符号つき) と `u8` (符号なし)、とかはよくある話
  - 浮動小数点型 (`f32`, `f64`。基本は `f64`)
  - 論理値型 (`bool`)
  - 文字型
- 複合型
  - タプル型
  - 配列型

### [3.3 関数の動作法](https://doc.rust-jp.rs/book/second-edition/ch03-03-how-functions-work.html)
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

### [4.1 所有権とは？](https://doc.rust-jp.rs/book/second-edition/ch04-01-what-is-ownership.html)

- その前に、スタックとヒープ
  - スタック：Last In, First Out
    - 例：お皿の山に皿を足したり取り除いたりするとき、常に一番上から
    - データはすべて既知の固定サイズ
  - ヒープ
    - ヒープにデータを置くとき、空の領域を見つけ、使用中にし、ポインタを返す
    - ポインタを追って実データにアクセスするのでスタックより低速
  - スタックは物理的にも近い位置にデータがある
- 所有権が解決する問題
  - どの部分のコードがどのヒープ上のデータを使用しているか把握すること
  - ヒープ上の重複するデータを最小化すること
  - メモリ不足にならないようにヒープ上のデータを掃除すること
- 文字列リテラルと`String`型の例
  - 文字列リテラルは不変。コンパイル時にバイナリファイルにハードコードされる -> 高速で効率的
- ヒープに確保したメモリを忘れずに解放するのは難しい -> Rustは変数がスコープを抜けたら自動的に解放
- ムーブ
  - ある変数を別の変数に束縛すると、元の変数は無効になる

```rust
let s1 = "hello";
let s2 = s1 // このタイミングでs1はs2に"ムーブされる"

println!("{}, world!", s1); // error
```

- クローン
  - ヒープデータのdeep copyが必要ならば、`clone`を使う
- 関数に値を渡したり、関数の戻り値を受け取ったりすることで、所有権は移動する

```rust
fn main() {
    let s = String::from("hello");  // sがスコープに入る

    takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない

    let x = gives_ownership();

    println!("{}", x);
    // println!("{}", s);  // ここでsを参照するとエラー

} // ここでs,xがスコープを抜ける。xはドロップされるが、sの値はムーブされてるので、何も特別なことはない。

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  // 

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string  // 呼び出し元関数にsome_stringがムーブされる
}
```
