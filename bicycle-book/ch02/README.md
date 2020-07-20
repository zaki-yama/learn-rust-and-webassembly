2 はじめてのRustプログラム
=======================


### 2-4 RPN計算機プログラムとデバッガによる実行

- RPN: Reverse Polish Notation、逆ポーランド記法
- スタックと相性がいい

#### 用語
- メソッド呼び出し構文
  - ref. `exp.split_whitespace()`
  - `exp.split_whitespace()` はコンパイル時に `split_whitespace(&exp)` という関数呼び出しとして解釈される
- クロージャ
  - ref. `|x, y| x + y`
  - `|<引数>| <式>` という構文
  - `fn()` と違い、引数と戻り値の型を省略できる
- トレイト境界
  - ref. `fn apply2<F>(..., fun: F) where F: Fn(f64, f64) -> f64`
  - ジェネリクスの型パラメータとして受け付けられる型の範囲に境界を定めることができる
  - Javaだと`T extends ...`、C#だとtype constraints


### 2-5 ツールチェインの補足情報

- ティア
  - Rust は多くのプラットフォームをサポートしているが、そのサポートの度合いによって何段階かのTierがある
  - Tier1 が一番サポートが手厚い。Linux, Mac, Win(MSVC), Win(GNU ABI)の4つ
- リリースサイクル
  - Chrome のサイクルに似ている
  - 6週間ごとのStable
  - Nightly(毎日), Beta(6週間ごと、Stableより6週間早い)
- エディション
  - 後方互換性のない変更をリリースするためのしくみ
  - Cargo.toml にクレートごとに edition を指定できる
  - これにより、依存ライブラリが新しい edition に対応するのを待たず、自分のソースは新しい edition で使われた機能を利用する、みたいなことができる
- rustup のその他の機能
  - 複数バージョンのRustツールチェインの管理
  - ...
- Cargo の主なコマンド
  - npm 的にパッケージ管理するには `cargo-edit` というカスタムサブコマンドを使えばいい
