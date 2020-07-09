実践Rust入門
===========

- 正誤表
  - https://gihyo.jp/book/2019/978-4-297-10559-4/support
- サンプルプログラム
  - https://github.com/ghmagazine/rustbook


### 疑問

- ✅ベクタをforループで回すとき。 `iter()` と `into_iter()` とは？
  - 7-9-10 イテレータと所有権
- structとimplとtrait
- 4-2-7 生ポインタ　は「メモリ安全でない」とはどういうこと？
- 4-2-8 関数定義型(fn item type)とは？「関数定義型は関数ごとに異なる型になる」とは？
- 4-3-4 文字列スライス(str), char, Stringの関係性
- ベクタやStringを「ユーザ定義型」と呼ぶのはなぜか
  - 元々備わっているんだからプリミティブ型のように思える
- 6-5-4 メソッドの「レシーバ」とは？
  - 第一引数の `&self` のこと
- [p272] ファットポインタ
- [p279] 可変の借用(&mut)経由では値の所有権を一方的に奪うことはできませんが、所有権を交換する、つまり別の値と交換するのならできるのです
  - `str::mem::replace`
- 参照、借用、ムーブらへんの用語
  - 値の参照を得ることを借用と呼ぶ (p263)
  - "関数の引数に参照を取ることを借用と呼びます"
    - [参照と借用 - The Rust Programming Language](https://doc.rust-jp.rs/book/second-edition/ch04-02-references-and-borrowing.html)
- クレート、トレイト、パッケージ
- `use` って一番上にまとめたほうがいいんじゃないの？


### 用語

- `#[derive(Debug)]`: 自動導出
  - 8-1-7
  - 構造体にこのアノテーションをつけると、いくつかの標準ライブラリのトレイトを自動で実装できる
  - 導出可能なのは `Clone、Copy、Debug、Default、Eq、Hash、Ord、PartialEq、 PartialOrd`
- `?` 演算子
  - `Option<T>` や `Result<T, E>` を返す関数に対して使用する
  - 結果が `Some(値)` や `Ok(値)` ならアンラップし、 `None` や `Err(E)` なら早期リターンする
- コンビネータ・メソッド: `map()` や `and_then()`
  - p375 に詳しい解説
