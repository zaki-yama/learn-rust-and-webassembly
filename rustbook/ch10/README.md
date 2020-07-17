第10章 パッケージを作る
====================

- デフォルトでは以下のように認識する
  - `lib.rs` をlibクレートのエントリポイント
  - `main.rs` または `bin/` ディレクトリ下のファイルをbinクレートのエントリポイント


### 10-2 ドキュメントを書く

- ドキュメンテーションコメント(`//` は普通のコメント)の種類
  - コメント直後のアイテムにつくか、上位のアイテムにつくか
  - 1行か、複数行か

```rust
/// 構造体
pub struct Struct {
  /** フィールド */
  pub field1: String;
}

/** 複数行コメント

行頭に * をつけるとMarkdownのリストと認識される
*/
pub enum Enum {
  /// 列挙子
  Variant,
}
```

```rust
/*! (ファイルの先頭に記述する
クレートです
*/

pub fn function2() {
  //! 関数その2
}
```

- ドキュメンテーションコメントの使い分け
  - `///`: アイテムのドキュメントに
  - `//!`: クレートや、ファイルやディレクトリに切り出されたモジュールのコメントに
  - `/** ~ */` や `/*! ~ */` はスタイルガイドでは非推奨
- ドキュメントはMarkdown書式
- Panics: Errors: Safety: Examples: は特別なセクション名
- **ドキュメント内のコードブロックはテスト時に同時にコンパイル、実行される**

### 10-3 テストの追加

- `assert!(expr)`, `assert_eq!(left, right)`, `assert_neq!(left, right)` はいずれもフォーマット引数を取れる

10-3-3 テストを書く場所

- 書ける場所としては4つある
  1. プログラム中
  2. プログラム中のテスト用のモジュール内
    - `#[cfg(test)]` アトリビュートをつけるとテストモードでコンパイルしているときしかコンパイルされない
  3. テスト専用のディレクトリ内
    - `tests` 直下にテストを書くと、Cargoがそれらをテストクレートとしてコンパイルしてくれる
      - NOTE: 直下のみ
  4. ドキュメンテーションコメント内
- 3だけが別のクレートとして扱われる
- 使い分けはどうするか？
  - クレート内テストはホワイトボックステスト
  - クレート外テストはブラックボックステスト

### 10-4 パッケージを公開するために
- Cargo.lock は bin クレートであればコミットすべきだが、lib クレートではライブラリ利用者の使う他のライブラリとの組み合わせがあるのでコミットすべきでない
  - `cargo new` コマンドはそれも加味した上で `.gitignore` を生成してくれるので、気にしなくてOK

#### CIを GitHub Actions でやりたい

- 日本語で参考にできそうな記事
  - [RustのLinux/Windows/macOS向け64bitバイナリをGitHub Actionsで生成する - Qiita](https://qiita.com/dalance/items/66d97c252b8dd9c96c29)
  - [GitHub ActionsでRustプロジェクトをクロスビルドしてリリースする - 詩と創作・思索のひろば](https://motemen.hatenablog.com/entry/2019/11/github-actions-crossbuild-rust)
- 基本は [actions-rs](https://github.com/actions-rs) を使うのがよさそう
- https://github.com/actions-rs/meta にレシピ集があるのでそれを参考にする
  - matrix 使ったやつは https://github.com/actions-rs/meta/blob/master/recipes/matrix.md
- Todo
  - [ ] actions-rs/toolchain の `profile` とは
  - [ ] actions-rs/toolchain の `override: true` はつけるべきか
  - [ ] GitHub Actions での `allow_failures` 相当のオプションは
    - [GitHub ActionsでTravis CIのallow_failures的なやつをやりたい - くりにっき](https://sue445.hatenablog.com/entry/2020/01/24/125100)
    - `include` でオプションを渡す
  - [ ] actions-rs/cargo 使う必要あるか

- `[badges]` セクションに `github-actions` は対応してるっぽいが、ドキュメントにない 
  - [Allow GitHub Actions badges in the badges section · Issue #8076 · rust-lang/cargo](https://github.com/rust-lang/cargo/issues/8076)
    - -> 対応してなかった。PRはcloseされてる。 `badges` セクション自体が deprecated らしい
    - ref. [Remove badges from crate lists and details · Issue #2436 · rust-lang/crates.io](https://github.com/rust-lang/crates.io/issues/2436)
- バッジはかわりにREADMEに置く
  - ref. [クレートを公開する前に埋めておくべきメタデータ | κeenのHappy Hacκing Blog](https://keens.github.io/blog/2017/12/03/kure_towokoukaisurumaeniumeteokubekimetade_ta/)

```markdown
[![<YOUR-CRATE-NAME> at crates.io](https://img.shields.io/crates/v/<YOUR-CRATE-NAME>.svg)](https://crates.io/crates/<YOUR-CRATE-NAME>)
[![<YOUR-CRATE-NAME> at docs.rs](https://docs.rs/<YOUR-CRATE-NAME>/badge.svg)](https://docs.rs/<YOUR-CRATE-NAME>)
```

- 1つめは https://shields.io/ で "rust" と検索するといくつか出てくる
- 2つめは https://docs.rs/about の Badges セクション参照

- `.github` ディレクトリが cargo package したときに含まれてしまう問題
