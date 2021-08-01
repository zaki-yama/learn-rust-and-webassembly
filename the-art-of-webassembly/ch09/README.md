# Chapter 9: Optimizaing Performance

<!-- TOC -->

- [Using a Profiler](#using-a-profiler)
  - [Chrome Profiler](#chrome-profiler)
  - [Firefox Profiler](#firefox-profiler)
- [wasm-opt](#wasm-opt)
  - [Running wasm-opt](#running-wasm-opt)
  - [Looking at Optimized WAT Code](#looking-at-optimized-wat-code)
- [Strategies for Improving Performance](#strategies-for-improving-performance)
- [Comparing the Collision Detection App with JavaScript](#comparing-the-collision-detection-app-with-javascript)
- [Hand Optimizing WAT](#hand-optimizing-wat)
- [Logging Performance](#logging-performance)
- [More Sophisticated Testing with benchmark.js](#more-sophisticated-testing-with-benchmarkjs)
- [Comparing WebAssembly and JavaScript with `--print-bytecode`](#comparing-webassembly-and-javascript-with---print-bytecode)
- [Summary](#summary)

<!-- /TOC -->

この Chapter で学ぶこと

- プロファイラツールの使い方
- WebAssembly と似たような JS とのパフォーマンス比較
- WebAssembly のパフォーマンスを向上させる戦略
  - 関数のインライン化、乗算・除算をビットシフトに置き換え、Dead Code Elimination (DCE)
- `console.log`と`Date.now`を使ったアプリケーションパフォーマンスの測定
- V8 の中間表現(Intermediate Representation: IR)バイトコード

## Using a Profiler

- 異なるタイプの最適化の間でトレードオフを行う必要があることも多い
  - たとえば、ユーザーがアプリケーションを可能な限り早く使い始められるよう TTI を向上させるか、ピーク時のパフォーマンスを優先するか
- DevTools の Performance タブの概要

### Chrome Profiler

- JavaScript Heap Memory
  - JS では GC があるのでメモリを気にする必要がないと考える開発者もいるが、残念ながらそうではない
    - GC されるより速くオブジェクトを作成している可能性もあるし、必要以上にオブジェクトへの参照を持っており、JS からは削除していいのかわからない状態になってる可能性もある
  - ヒープメモリについては、2.2MB ぐらいまで増加した後 GC が起き、1.2MB ぐらいまで減少していた
- Following the Memory Allocation
  - ヒープの増加が一貫していることから、毎フレームレンダーごとにメモリ割り当てが行われている可能性が高いと推測される
  - WebAssembly の関数呼び出しより `ctx.putImageData` の方がヒープ増加に寄与していた
    - これはビルトイン関数なので最適化のしようがない
    - メモリアロケーションが問題なのであれば別の手段を考える
- Frames
  - 18fps ぐらい（自分の環境だと 30fps ぐらい)
- Bottom-Up
  - Bottom-Up タブはこういったデータが見られる
    - アプリ内で呼ばれた関数
    - それらの実行時間の total time
    - Self Time (関数からさらに呼ばれた関数の実行時間を除外した、その関数自身の実行時間)
  - 🤔 "It’s a bit disappointing that Chrome doesn’t indicate which function it calls inside the WebAssembly module" と書いてるけど、自分の環境だと `wasm-function[4]` みたいな表示だった & Sources タブに飛んでだいたいどの関数かあたりつけることはできた

### Firefox Profiler

skip

## wasm-opt

- `wasm-opt`: `wat-wasm` か Binaryen.js をインストールすると使える CLI

### Running wasm-opt

- ダウンロードサイズとパフォーマンスのどちらにフォーカスするかを optimizer に伝えるためにフラグを使う
- フラグは、Emscripten や AssemblyScript といった、Binaryen を使ったツールチェインでも同じ
- Optimizing for Download Size
  - `-Oz` と `-Os`: `-Oz` の方が生成される WebAssembly ファイルのサイズは小さいが、実行に時間がかかる
    - 大きいプロジェクトとかだと `-Os` を使う選択肢がある
- Optimizing for Execution Time
  - ゲームを作っている場合、ダウンロードタイムより fps の向上の方に関心があるだろう
  - `-O1, -O2, -O3`
    - 数字が大きくなるにつれ最適化のレベルが上がるが実行時間も長くなる

### Looking at Optimized WAT Code

- `wasm2wat` で最適化済みの .wasm ファイルを WAT に変換して中身を見てみる
- 最適化の例
  - 関数名や変数名をなくす
  - 関数の数を減らす（インライン化）
  - 関数内の変数の数を減らす
  - 2^n との掛け算はビットシフトに変える

## Strategies for Improving Performance

- Inlining Functions
  - サンプルコード: inline.wat
  - 🤔 `wasm-opt` かけると `(type` が登場するのなんでだろ？
    - `(type` 自体は p61 に登場
      > The last expression is a `type` expression which defines the signature of the functions in the table. I have to provide this `$returns_i32` type as a static parameter to `call_indirect`
- Multiply and Divide vs. Shift
  - サンプルコード: pow2_mul.wat
  - 2 の階乗の掛け算をビットシフトに変更する
- Combining Constants
  - 定数をひとまとめにする
- DCE
  - Dead Code Elimination

## Comparing the Collision Detection App with JavaScript

- Chapter 8 で作った衝突判定アプリの JS 版を作ってパフォーマンス比較してみよう
- 🤔 Wasm 版と比べて width, height が 1024 なのはなんでだろ？

## Hand Optimizing WAT

- 手作業でアプリの最適化をやったらだいぶ高速化したよ、実際のコードは https://wasmbook.com/collide.html とか https://wasmbook.com/collide.wat 見てね、とだけ

## Logging Performance

- シンプルな計測方法は JS の`Date` と `console.log` 使う方法
- ただし、Wasm から JS の関数は直接呼び出せないので渡してあげる必要があり、その分のオーバーヘッドがある
- というわけでオーバーヘッドがどの程度なのかサンプルコードを書いてみる( mod_and.wat )
  - 🤔 似たようなこと以前の章でやらなかったっけ？
- オーバーヘッドを減らすため、ループを Wasm 内でやるようにする (mod_and_loop.wat)

## More Sophisticated Testing with benchmark.js

- benchmark.js (https://github.com/bestiejs/benchmark.js) を使ったパフォーマンス測定方法
- "wasm-opt" の節でやった pow2_mul.wat の実装方法を何パターンか用意して benchmark.js 使ってパフォーマンス測定
  - pow2: オリジナルのやつ
  - pow2_reverse: div と mul の順序を逆にしたやつ
  - pow2_mul_div_shift: mul と div を順序を逆 & ビットシフトにしたやつ
  - pow2_mul_div_nor: mul と div をビットシフトにしたやつ
  - pow2_opt: wasm_opt が生成したやつ
- 面白いことに、結果は一番悪いのが wasm_opt が生成したやつだった

## Comparing WebAssembly and JavaScript with `--print-bytecode`

- V8 は JS を IR bytecode (アセンブリ言語や WAT に似ている)にコンパイルする
- IR はレジスタやアキュムレータを使うが、特定のマシン固有ではない
- この IR を使って、JIT コンパイラを実行した後の JS コードと Wasm のコードを比較できる
- `$ node --print-bytecode --print-bytecode-filter=bytecode_test ch09/print_bytecode.js`
- "Instead of a stack machine, the bytecode that the V8 engine generated is for a virtual register machine with an accumulator register"
  - スタックマシンの代わりに、V8 が生成するバイトコードはアキュムレータレジスタを持った仮想レジスタマシン用である
- "Accumulator machines have one general-purpose register where the accumulator does all of the math instead of doing it in the other registers."
- opCodes の例
  - `a` はアキュムレータ、 `r` はレジスタを指す
  - `LdaZero`: loads (`Ld`) the accumulator (`a`) with 0 (`Zero`)
  - `Star r0`: stores (`St`) the value in the accumulator (`a`) into a register (`r`) and then passes in `r0` to define that register
- レジスタの値を直接セットすることはできないので、一旦アキュムレータに load した後にレジスタに移すということが必要になる

## Summary

- ブラウザの Profiler を使ったパフォーマンス測定(Summary, JS Heap Memory, fps)
- Binaryen.js のインストール、 `wasm-opt` を使った最適化
- WebAssembly のパフォーマンスを最適化するためのいくつかの戦略
- JS 版とのパフォーマンス比較、benchmark.js を使ったパフォーマンス測定
- V8 が生成するバイトコード (IR bytecode) の読み方
