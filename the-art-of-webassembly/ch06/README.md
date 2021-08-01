# Chapter 6 Linear Memory

- linear memory とはなにか
- JS と Wasm の間でデータを共有するために linear memory をどう使うのか
- JS 内で linear memory をどう作成するのか

## Linear Memory in WebAssembly

- ローカル変数をスタックに格納するというのは、stack pointer をインクリメント／デクリメントすればいいのでシンプル
- しかし、WAT における制限として、スタックを使えるローカル変数は数値型 4 種類しか使えない
- C における `malloc` や C++や JS における `new` のようなアロケーションコマンドは、ヒープ上に割り当てを行いますが、これらの言語に含まれるメモリ管理ライブラリは、必要なメモリブロックを格納するのに十分な大きさのヒープ上のメモリの空きセクションを探す必要があります。その結果、図 6-2 に示すように、割り当てられたメモリセグメントが未割り当てのメモリで区切られる、メモリフラグメンテーションが発生する可能性があります。
- WebAssembly の linear memory は pages と呼ばれる大きなチャンクにアロケートされ、一度アロケートされるとデアロケートすることはできない
- WebAssembly のメモリはアセンブリ言語っぽいメモリ管理である: あなたが決めたページ数を WebAssembly モジュールにアロケートしたら、どこに・何のためにメモリを使っているのかを追跡する必要がある

### Pages

- 1 ページ 64KB (現在はこのサイズは変更不可)
- 最大ページ数は 32,767、メモリサイズの最大値は 2GB

#### Creating Pages in Your Code

```wat
(memory 1)
```

#### Creating Memory in the Embedded Environment

```js
const memory = new WebAssembly.Memory({ initial: 1 });
```

#### Pointers

- `i32.store <address> <value>` で linear memory 内の指定したアドレスに値をセットする
- `i32.load <address>` で値を取り出す

## JavaScript Memory Object

- memory は 0 で初期化されるぽい
  - " If we don’t initialize it with a value, the memory buf- fer begins with all data set to 0"
- 🤔 local の$index は何で初期化されてる？ -> たぶん 0

## Collision Detection

- 前節では memory buffer の作成は JS、初期化は wasm だったが、ここでは初期化も JS 側でやってみる
- 2 つの円の衝突判定を行うサンプル

### Base Address, Stride, and Offset

- linear memory を JS で扱う際は typed array として扱った
- Wasm モジュール内では、linear memory はメモリヒープや、巨大なバイト配列に近い
- あるデータ構造の配列を作るためには、
  - starting address (base address)
  - stride (distance in bytes between each structure)
  - offset of any structure's attributes (how far into a structure can we find our attribute)
- 今回のデータ構造は 4 つの attributes:
  - x 座標
  - y 座標
  - 半径 radius
  - ヒットフラグ

## Summary

この章でやったこと

> In this chapter, you learned what WebAssembly linear memory is and how to create it from within the WebAssembly module or JavaScript. Next,
> we initialized the linear data from within the WebAssembly module and accessed it from JavaScript. Then we created data structures within linear memory using a base address, stride, and attribute offsets, and initialized these data structures from within JavaScript using random data.
