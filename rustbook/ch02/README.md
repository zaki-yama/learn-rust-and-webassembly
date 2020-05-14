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
