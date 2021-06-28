# Chapter 5 Strings in WebAssembly

本章でやること

- ASCII と Unicode 文字列
- linear memory から JS でどのように文字列を取得するか
- 2 つのポピュラーなメソッド：null-terminated strings and length-prefixed strings
- 文字列のコピー

### ASCII and Unicode

- ASCII: 7 ビットで 128 種類の文字列をサポート + 1 ビットはエラーチェック用
- UTF: 7, 8, 16, 32 ビットがある(UTF-\*)
- 本書でカバーするのは ASCII/UTF-7 のみ

### Strings in Linear Memory

- Wasm から JS に文字列を渡すには（Chapter 2 のおさらい）：
  - 文字列の配列を memory 内に作る
  - memory buffer 上の位置を示す 32 ビットの integer を JS にわたす
  - この方法だと文字列の終わりがわからない。どうする？
    - C だと null-terminating byte (末尾に null 文字列 `0`を入れる）で実現してる
- 文字列の終わりを判断する方法 3 パターン
  - Passing the String Length to JavaScript
    - JS に文字列の長さを渡してあげる
  - Null-Terminated Strings
    - 文字列の終端に null 文字列 `\0` を入れる
  - Length-Prefixed Strings
    - 文字列の最初に文字列自体の長さを埋め込んでおく

## Copying Strings

- linear memory 上のある位置にある文字列を別の場所にコピー
- 一番シンプルなのは、1 バイトずつループさせながら別の場所に保存することだが、これは遅い
- より効率的なのは、64 ビット integer に 8 バイト分の文字列を一度にコピーして保存することだが、すべての文字列が 8 バイトの倍数とは限らない
- -> ここでは両者を組み合わせて使う
  - たとえば、文字列が 45 文字なら最初の 40 文字は 8 バイトごと x 5 回の処理でコピーし、残りの 3 バイトは byte-by-byte でコピーする

### Byte-by-Byte Copy
