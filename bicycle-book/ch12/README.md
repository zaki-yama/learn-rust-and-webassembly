第12章 FFI
==========

- FFI は Foreign Function Interface の略。Rust から他言語の API を使う、または他の言語から Rust を呼び出すこと
- 本章では C から Rust、Rust から C 両方紹介

```rust
use std::os::raw::c_double;


// インポートするCの関数群はextern "C" { .. } で囲む
extern "C" {
  fn cos(x: c_double) -> c_double;
}

fn main() {
  // C の関数はRustの管理下にないのですべてアンセーフとして扱われる
  unsafe {
    // Rustの関数のように呼び出せる
    println!("{}", cos(1.5));
  }
}
```

- Cの関数に対応するRustのシグネチャを書くのはプログラマの責任
- 12-1-5 `build.rs` という名前のファイルを置くと、ビルド時のスクリプトとして自動実行されるようになる
