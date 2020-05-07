### 1.2 

- `fmt::Display` と `fmt::Debug` の違い
  - https://doc.rust-lang.org/std/fmt/#fmtdisplay-vs-fmtdebug
  - `fmt::Display`
    - 型が UTF-8 文字列で表現できることをアサートする
    - すべての型に実装されているわけではない
  - `fmt::Debug`
    - すべての型に実装する必要がある

### 1.2.1 デバッグ

`
