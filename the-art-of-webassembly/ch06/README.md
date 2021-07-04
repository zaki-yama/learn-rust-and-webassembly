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

## JavaScript Memory Object
