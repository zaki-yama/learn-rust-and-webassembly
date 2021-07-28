# Chapter 10 Debugging WebAssembly

この Chapter で学ぶこと

- WAT コードをデバッグするためのテクニック
- console へのロギング、alert の使い方、stack trace のロギング
- Chrome や Firefox の debugger の使い方（両者の違いや制限も含め）

- source map: Emscripten などのツールチェインは、WebAssembly バイナリと元の C++ このコードとの source map を作ってくれる
- この本執筆時点では、wat2wasm は source map を生成してくれない
  - デバッグできなくなるわけではないが、変数名などは失われる

## Debugging from the Console

- WAT の要所要所で console.log する
- 識別しやすいように message id という名の index をログ出力する箇所に連番ふっていく、というかなり泥臭いデバッグ手法
