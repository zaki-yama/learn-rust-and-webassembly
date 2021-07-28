# Chapter 11 AssemblyScript

この Chapter で学ぶこと

- AssemblyScript で Chapter 1 のときに作った `AddInt` と似た `AddInts` を作る
- AssemblyScript で hello world
- length-prefixed strings を AssemblyScript でやると文字列のやり取りがどれだけ簡単か見てみる
- AssemblyScript で OOP

- AssemblyScript は JS/TS のような高レベルな言語でありながら、WAT のような低レベルのメモリ命令も備えている

## AssemblyScript CLI

- 🤔 拡張子 `.ts` だと TypeScript だと認識されるので `i32` みたいな型でエディタ上エラー出るけどそういうもんなの？
- 🤔 `type $i32_i32_=>_i32` なんてあったっけ？

## Hello World AssemblyScript

- 生成される .wat ファイルは微妙に違う
  - `(global $~lib/memory/__stack_pointer (mut i32) (i32.const 16444))`
- as_hello.js 見るとなんか文字列のやりとり全然簡単になってない... -> AssemblyScript Loader ですよ、みたいな流れ

### Hello World with the AssemblyScript Loader

- `@assemblyscript/loader` 使うと `__getString` のような便利関数が使える
- 🤔 `let module = null;` で怒られるの謎
  - "SyntaxError: Identifier 'module' has already been declared"

### AssemblyScript String Concatenation

- JS 側から文字列を渡すには？という話
- コンパイル時に `--exportRuntime` オプションをつけると `__allocString` 関数が JS から使える
