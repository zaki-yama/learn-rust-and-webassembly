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
