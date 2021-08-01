# Chapter 7 Web Applications

> This chapter will help you understand how WebAssembly interacts with the DOM through JavaScript.

- `add_message.wat`
  - 2 つの数値を足した結果を DOM に表示する。WebAssembly 側からは直接 DOM 操作できないので、DOM を書き換える JS 側の関数を import して使う
- `hex_and_binary.wat`
  - 画面から入力された数値を 16 進数と 2 進数に変換して画面に表示する
  - HTML タグも含めた文字列を WebAssembly 側で linear memory に格納して、JS 側では決まったアドレスから読み込むことで文字列を取得する

## The DOM

一般的に、Web アプリケーションの WebAssembly 部分は、数値データの処理に焦点を当てるべきだが、DOM ではデータ処理のほとんどが文字列操作になるだろう。WebAssembly 内での文字列操作のパフォーマンスは、そのタスクに使用するライブラリに完全に依存する。このような理由から、DOM の重い作業は通常、アプリケーションの JavaScript 部分に留めておくのがベスト。

> As a general rule, the WebAssembly portion of a web application should focus on work­ ing with numeric data, but with the DOM most of the data processing will likely be string manipulation. The performance of string manipulation from within WebAssembly is entirely dependent on the library you use for the task. For this reason, DOM heavy work is usually best kept in the JavaScript portion of the app.

## Setting Up a Simple Node Server

## Our First WebAssembly Web Application

## Hex and Binary Strings

- Wasm 側で DOM 操作は直接はできないので、
  - Wasm 側の関数で
    - HTML タグを含む文字列を作る
    - それを linear memory に格納する
    - 作成された文字列の長さを返す
  - JS 側から Wasm の関数を呼び出し、返り値の文字列長 `len` を使って `new Uint8Array(memory.buffer, 1024, len)`
  - ...のようにする
