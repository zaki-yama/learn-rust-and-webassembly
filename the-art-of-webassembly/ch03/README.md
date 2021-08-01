# Chapter 3 Functions and Tables

この Chapter で学ぶこと

- WebAssembly の関数について学ぶ
  - いつ、どのように JS や他の WebAssembly モジュールから関数を import するのか
  - どのように WebAssembly の関数を export するのか
- table について

<!-- TOC -->

- [When to Call Functions from WAT](#when-to-call-functions-from-wat)
- [Writing an `is_prime` Function](#writing-an-is_prime-function)
  - [Passing Parameters](#passing-parameters)
- [Declaring an Imported Function](#declaring-an-imported-function)
  - [Performance Implications of External Function Calls](#performance-implications-of-external-function-calls)
  - [Function Tables](#function-tables)

<!-- /TOC -->

## When to Call Functions from WAT

- `(export)`
- JS から Wasm の関数の呼び出しはオーバーヘッドがあるので、小さなタスクを実行するだけの関数であれば export すべきでないよ。JS でやった方がいい
- Wasm の関数は大量のデータを何回もループで処理するようなものに適している
- デバッグについては Chapter 10
- パフォーマンスチューニングの過程で、元々関数として切り出してたものをインライン化することもあるかもしれない
  - パフォーマンス・チューニングについて詳しくは Chapter 9 で

## Writing an `is_prime` Function

### Passing Parameters

- `local.tee`: The local.tee command is like the local.set command in that it sets the value of the variable you pass to it to the value on top of the stack

## Declaring an Imported Function

### Performance Implications of External Function Calls

- "When you call a JavaScript function
  in WAT, you lose some cycles to overhead. This number isn’t extremely large, but if you execute an external JavaScript function in a loop that iterates 4,000,000 times, it can add up."
- 2 つのベンチマーク比較
  1. 内部で Wasm の関数を 4_000_000 回呼び出し
  2. 内部で JS から import した関数を 4_000_000 回呼び出し

### Function Tables

- "Currently, tables only support the anyfunc type (anyfunc is a generic WebAssembly function type), but in the future they might support JavaScript objects and DOM elements as well."
- "Unlike import objects, JavaScript and WebAssembly can dynamically change tables at runtime."
- function table 経由での関数呼び出しは indirect なコールになるためパフォーマンスコストがある
- "However, you cannot add a JavaScript function to a function table from within JavaScript. There is a WebAssembly.Table function set that allows you to set functions in a table, only with a function defined in a WebAssembly module. We can work around this restriction by importing the JavaScript function into a WebAssembly module and adding it to the table there."
  - table_export.wat で一度 import して table にセットしてるのはこれが理由

```
js_table_test time=67
js_import_test time=52
wasm_table_test time=26
wasm_import_test time=19
```

- オチとしては
  - JS より Wasm のほうが速い
  - table 経由より直接呼び出しのほうが速い
