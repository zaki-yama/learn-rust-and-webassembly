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
- 式は値を返すので、関数の最終行が `;` なしの場合、それが戻り値となる

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

### [4.2 参照と借用](https://doc.rust-jp.rs/book/second-edition/ch04-02-references-and-borrowing.html)

- デフォルトで、関数の引数に渡した値はムーブしてしまうので、呼び出し後もその値を使いたい場合はタプルなどで引数自体も返す必要がある

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    // ここではもうs1は参照できない
    println!("The length of {} is {}", s2, len);
}
```

- 代わりに、引数としてオブジェクトへの **参照** を渡すことができる
  - 関数の引数に参照を取ることを **借用** と呼ぶ

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

![](images/trpl04-05.svg)

- 借用した参照を変更するには変数宣言および参照を渡している箇所に`mut`をつける
  - 特定のスコープで、ある特定のデータに対しては、一つしか可変な参照を持てない
- 可変な参照と不変な参照を組み合わせることはできない

```rust
let s = String::from("hello");

let r1 = &s;
let r2 = &s; // ok
let r3 = &mut s;  // error
```

### [4.3 スライス型](https://doc.rust-jp.rs/book/second-edition/ch04-03-slices.html)

- 文字列の何バイト目から何バイト目への参照、を文字列スライスという
  - `&hoge[0..5]` のようにする
  - 型は`&str`で表す

### [5.1 構造体を定義し、インスタンス化する](https://doc.rust-jp.rs/book/second-edition/ch05-01-defining-structs.html)

- TypeScriptの`type`のイメージ

```rust
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}
```

- `{ email: email }`を`{ email }`と省略できるあたりはJSと同じ
- 可変の場合`mut`つけるのは同じ。一部のフィールドのみ可変にすることはできない
- 構造体更新記法: `..obj1`とすると他のインスタンスからインスタンスを生成できる

```rust
let user2 = User {
  email: String::from("another@example.com"),
  username: ... ,
  ..user1
};
```

- タプル構造体：フィールド名がなく、タプルに似た構造体
  - 例： `struct Color(i32, i32, i32);`


### [5.3 メソッド記法](https://doc.rust-jp.rs/book/second-edition/ch05-03-method-syntax.html)

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height
    }

    // 関連関数
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
```

- 構造体を定義したあと、`impl`内でメソッドを定義する
- メソッドの第一引数が必ず`self`なのはPythonぽい
  - が、所有権の考え方はここでも同様
- **関連関数**：`self`を引数に取らない、staticメソッド的なもの
  - `Rectangle::square(30)`のように`::`で呼び出す
