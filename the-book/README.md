[The Rust Programming Language](https://doc.rust-jp.rs/book/second-edition/foreword.html)
===============================

## キーワード
- 式指向言語
- crate
- trait

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

![](\_images/trpl04-05.svg)

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


### [7.1 Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)

- package: 1つまたは複数のクレートから成る。 `Cargo.toml`を含む
- crate

### [7.2 Defining Modules to Control Scope and Privacy](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)

### [7.3 Paths for Referring to an Item in the Module Tree](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)

- デフォルトはすべてprivate
- 親 -> 子を参照することは基本できないが、子 -> 親はたどれる
- 構造体は`struct`に`pub`をつけても内部のフィールドはprivateのまま
- enumは`pub enum`とすれば全変数にアクセス可能

### [7.4 Bringing Paths into Scope with the `use` Keyword](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html)

- `use foo::bar::baz` と書くと `foo::bar::baz::hoge()` が `baz::hoge()` と書ける
- `use foo:bar::baz::hoge` とせず `::baz` までにしておき、使うときに親のモジュールまで記述する(`baz::hoge()`)方が良しとされる
  - 理由は、ローカルで定義された関数と見分けがつかないため
- 一方、structの場合はフルパス指定で`use`する方がベター
  - 強い理由はない。慣例的に
  - 複数のモジュールで名前がバッティングしない限り
- JSのモジュールシステムみたく`as 別名`も使える

### [7.5 Separating Modules into Different Files](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html)

### [8.1 Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)

- Vector: 同じデータ型の複数の値を格納するためのデータ型
- メモリ的にも近い位置に格納される
- 初期化は初期値を与える方法とそうでない方法がある

```rust
// 初期値がなく、ジェネリクスで型を指定する
let v: Vec<i32> = Vec::new();

// 初期値から推論する。vec!というマクロを使う
let v = vec![1, 2, 3];

// 初期値がない場合も、pushする値から推論される
let mut v = Vec::new();
v.push(5);
```

### [10.2 Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html)
- interfaceのようなもの

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

で定義し、

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

のように使う。`impl <trait> for <traitを継承するstruct>` となる。

interfaceのようにシグネチャだけ定義することもできるし、デフォルトの実装を定義することもできる

```rust
pub trait Summary {
  fn summarize(&self) -> String {
    String::from("(Read more...)")
  }
}
```
