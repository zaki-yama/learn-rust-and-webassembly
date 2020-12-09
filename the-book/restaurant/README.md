肥大化していくプロジェクトをパッケージ、クレート、モジュールを利用して管理する
=====================

https://doc.rust-jp.rs/book-ja/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

### パッケージ、クレート、モジュール

- パッケージ：1つ以上のクレートを持ち、ある機能を提供するための単位
  - 1つの `Cargo.toml` で管理する粒度
  - 0個か1個のライブラリクレートを持つ
  - バイナリクレートはいくらでも持って良い
  - ライブラリクレートとバイナリクレートをあわせて少なくとも1つのクレートを持っていないといけない
- クレート：ライブラリ(`lib.rs`)か実行可能ファイル(`main.rs`)を生成するモジュール群
  - ライブラリクレート： `src/lib.rs`
  - バイナリクレート： `src/main.rs`
    - または、 `src/bin` ディレクトリにファイルを置くことで複数のバイナリクレートを持つことができる
- モジュール： `mod` や `use` でまとめる単位

ので、ディレクトリ構成としては以下のようになる（後述するモジュール単位のファイルを除く）

```yaml
<package>
  - Cargo.toml
  - src/
    - lib.rs     ... 0 or 1
    - main.rs    ... 0 or 1
    - bin/       ... 0 or N (files)
      - bin_a.rs
      - bin_b.rs
      - ...
```

### モジュールの基本的なルール

#### `mod` キーワードでくくった範囲がモジュールとなり、スコープが分かれる

```rust
mod front_of_house {
    mod hosting {
      fn add_to_waitlist() {}
    }
}

mod back_of_house {}
```

これは、以下のようなモジュールツリーを形成する。

```
crate
  - front_of_house
    - hosting
      - add_to_waitlist
  - back_of_house
```


#### モジュールはパスを指定して呼び出す

- 絶対パスで呼び出す場合、 `crate` から始める
- 相対パスで呼び出す場合、 `self`, `super`, または今のモジュールの識別子を使う
- ファイルシステムのパスと考え方は近い

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();

    // 相対パス
    front_of_house::hosting::add_to_waitlist();
    self::front_of_house::hosting::add_to_waitlist();
}
```

#### モジュールはデフォルトで非公開。公開するには `pub` キーワードをつける

- モジュールの公開範囲と、モジュールの中身の公開範囲は別々に指定する

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();

    // 相対パス
    front_of_house::hosting::add_to_waitlist();
    self::front_of_house::hosting::add_to_waitlist();
}
```

- 構造体は各フィールドを個別に公開するかどうか決められる
- enum は公開するとそのヴァリアントはすべて公開される

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // このフィールドにはアクセスできない
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let meal = back_of_house::Breakfast {
        toast: String::from("Rye"),
        seasonal_fruit: String::from("blueberries"), // error
    };

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

#### `use` キーワード

- `use` キーワードを使うと、モジュール内の要素が自分のモジュール内にあるかのようパスを省略できる
- ファイルシステムにおけるシンボリックリンクのようなイメージ

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // before:
    // crate:front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
}
```

- 慣例的に、
  - 関数を`use`で持ち込む場合、親モジュールを`use`して`some_module::func()`と書く
    - パスを省略しつつ**関数がローカルで定義されていないことを明らかにできるため**
  - 構造体やenumを`use`で持ち込む場合、フルパスを書く
- `as` で別名をつけることもできる。モジュール間で名前の衝突を回避するときなどに使える

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

- `pub use` を使うと名前を再公開(re-exporting)できる
    - 下の例では、このライブラリクレートを使う外部のコードも `hosting::add_to_waitlist()` を使えるようになる

```rust
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;
```

#### モジュールを複数のファイルに分割する

https://doc.rust-jp.rs/book-ja/ch07-05-separating-modules-into-different-files.html

- `src/lib.rs` に `mod xxx;` という宣言だけ書くと、モジュールの中身は `src/xxx.rs` というファイルを見に行くようになる

```rust
// src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;
```

```rust
// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

- さらに、 `src/xxx.rs` 内で `mod yyy;` という宣言だけ書くと、モジュールの中身は `src/xxx/yyy.rs` というファイルを見に行くようになる

```rust
// src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;
```

```rust
// src/front_of_house.rs
pub mod hosting;
```

```rust
// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```

というわけで、ディレクトリ構成は以下のようになりそう。

```yaml
- src/
  - lib.rs
  - module_a.rs # サブモジュール含む
  - module_a/
    - sub_module_a.rs
  - module_b.rs # サブモジュール含まない
  - ...
```

- モジュールツリーとしては変わらないので、使う側の `use` 宣言や呼び出し方は何も変わらない


### `lib.rs` で定義したモジュールを `main.rs` で使うには？

ドキュメントでは触れられていなかった部分。
A. パスをパッケージ名から始める

```rust
// Cargo.toml
[package]
name = "restaurant"
version = "0.1.0"
```


```rust
use restaurant::hosting;

fn main() {
    hosting::add_to_waitlist();
}
```

こうすると使えるようになるが、根拠となるドキュメントは見つけられてない。  
Stackoverflow のこれは若干関係しそう。

[Rust modules confusion when there is main.rs and lib.rs - Stack Overflow](https://stackoverflow.com/questions/57756927/rust-modules-confusion-when-there-is-main-rs-and-lib-rs)

### 余談： `mod.rs`

「Rust モジュールシステム」とかでググると、モジュールを `xxx/mod.rs` というファイルで定義する方法が紹介されてる記事が見つかるが、

[Rustのモジュールの使い方 2018 Edition版 | κeenのHappy Hacκing Blog](https://keens.github.io/blog/2018/12/08/rustnomoju_runotsukaikata_2018_editionhan/)

を読む限り、これは 2015 Edition の頃の後方互換性のために用意されているらしい。

これ以上深追いはせず、素直に公式ドキュメントで案内されてる方法を使うことにする。
