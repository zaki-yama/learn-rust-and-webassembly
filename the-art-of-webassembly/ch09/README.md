# Chapter 9: Optimizaing Performance

<!-- TOC -->

- [Using a Profiler](#using-a-profiler)
  - [Chrome Profiler](#chrome-profiler)
  - [Firefox Profiler](#firefox-profiler)
- [wasm-opt](#wasm-opt)
  - [Running wasm-opt](#running-wasm-opt)
  - [Looking at Optimized WAT Code](#looking-at-optimized-wat-code)
- [Strategies for Improving Performance](#strategies-for-improving-performance)

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
- Combining Constants
  - 定数をひとまとめにする
- DCE
  - Dead Code Elimination
