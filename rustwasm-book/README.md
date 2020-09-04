Rust 🦀 and WebAssembly 🕸
===========================

https://rustwasm.github.io/docs/book/introduction.html

<!-- TOC -->

- [学び](#学び)
- [疑問](#疑問)
- [メモ](#メモ)
  - [2. Why Rust and WebAssembly?](#2-why-rust-and-webassembly)
  - [3.1 What is WebAssembly?](#31-what-is-webassembly)
  - [4. Tutorial](#4-tutorial)
    - [4.1 Setup](#41-setup)
    - [4.4 Implementing Life](#44-implementing-life)
    - [4.5 Testing Life](#45-testing-life)
    - [4.6 Debugging](#46-debugging)
    - [4.7 Adding Interactivity](#47-adding-interactivity)
    - [4.8 Time Profiling](#48-time-profiling)
    - [4.9 Shrinking .wasm Size](#49-shrinking-wasm-size)
    - [4.10 Publishing to npm](#410-publishing-to-npm)

<!-- /TOC -->

## 学び

- wasm-pack を前提としたアプリの開発方法
  - Rust 側は [wasm-pack-template](https://github.com/rustwasm/wasm-pack-template) を、フロントエンド側は [create-wasm-app](https://github.com/rustwasm/create-wasm-app) テンプレートを使った
- デバッグ方法
  - Rust 側のプログラムに `console.log()` を仕込む方法
  - Rust 側でパニック時にブラウザの `console.error` に出力する方法
- `web-sys` crate を使うと JS の Web API が Rust でも使える
  - e.g.) `console.log()` -> `web_sys::console::log_1()`
- テスト
  - `wasm-pack test --chrome --headless` でE2Eテスト

## 疑問
- wasm-pack-template の中身
- memory モジュールを使ったメモリの読み書き(Tour of WebAssembly もっかい読むとよさそう)
- `extern crate` って必要なの？
- `.wat` ファイル

## メモ
### 2. Why Rust and WebAssembly?

- ローレベルなコントロールとハイレベルなエルゴノミクス(?)
  - JS は信頼できるパフォーマンスを獲得しようともがいていた
    - JS の動的な型システムはガベージコレクションによる一時的な停止はその妨げになってる
- Small `.wasm` Sizes
  - `.wasm` はネットワークを通じてダウンロードされるので、コードサイズは重要
  - Rust はランタイムを持たないので、ガベージコレクターのようなextra bloatは含まない
- Do Not Rewrite Everything
  - 全部を置き換える必要はなく、パフォーマンスが重要な箇所を置き換えるところから始められる
- Plays Well With Others
  - Rust x WebAssembly は既存のJSツールとうまく連携する
    - ESModule をサポートするし、npm, Webpack, Greenkeeperなど気に入っているツールを使い続けることもできる
- The Amenities You Expect
  - Rust はモダンな言語なので開発者が期待するような以下のアメニティも揃えてるよ
    - cargo によるパッケージマネジメント
    - 表現力の高い(そしてゼロコストの)抽象化
    - コミュニティ

### 3.1 What is WebAssembly?

- WASMのキーワード: portable, compact, execute at or near native speeds
- WASMは以下2つのフォーマットがある
  - `.wat`: S-expressions という記法を使ったテキストフォーマット(WebAssembly Text)で、SchemやClojureなどのLisp系言語と似ている
  - `.wasm`: バイナリフォーマットで、wasm vitual machines で直接消費されることを意図している。  
  概念的にはELFやMach-O(?)と似ている
- Linear Memory
  - WASMはシンプルなメモリモデル
  - 単一の "linear memory"、本質的にはフラットなバイト列を扱う

### 4. Tutorial

#### 4.1 Setup

ツールの紹介

- `wasm-pack`
  - Rustから生成されたWebAssemblyのビルド、テスト、パブリッシュを一貫してやってくれるツール
- `cargo-generate`
  - Rust プロジェクトをテンプレートからはじめられるCLI
  - `create-react-app` とか `yeoman` 的なやつかな
- `npm`

#### 4.4 Implementing Life

Interfacing Rust and JavaScript

- JS の garbage-collected heap - `Object` や `Array`、DOM が格納される場所と、Rustの値が格納されるWebAssemblyの線形なメモリ領域はまったく異なる
- WASMはApril 2018まではこのgarbage-collected heapには直接アクセスできない(Interface Types proposalが実現すると変わることが期待されていたらしい)
  - 今は？🤔
- 一方JSはWASMの線形メモリ領域の読み書きが可能だが、スカラ値(`u8`, `i32`, `f64` etc.)の `ArrayBuffer` としてしか扱えない
  - WASMの関数も同様にスカラ値を引数で受け取って返すことしかできない
- `wasm_bindgen` は、このWASMとJSの境界をまたいで複合的な構造のデータをやり取りするための共通理解を定義する
  - Rustの構造体をboxingしたり、
  - 利便性のためにJSのクラス内のポインタをラップしてあげたり
- WASMとJSの間のインターフェースを設計する際、以下のようなことを最適化したい：
  1. WASMの線形メモリ領域への／からのコピーを最小化したい
    - 不要なコピーは不要なオーバーヘッドを引き起こす
  2. シリアライズとデシリアライズを最小化したい
    - コピー同様、オーバーヘッドになる
    - "opaque handles" (不透明なハンドル) とは？
- 一般的な経験則として、良いJS<>WASMのインターフェース設計は、long-livedなデータ構造はRustで実装し、それをopaque handlesとしてJS側に露出するというもの

Interfacing Rust and JavaScript in our Game of Life

- 避けるべき危険要因を列挙するところからはじめよう
  - 毎回のtickごとにすべてのuniverseをWASMの線形メモリ領域にコピーするといったことはしたくない
  - universeのすべてのセルのためにオブジェクトを割り当てる(allocate)ことはしたくないし、 nor do we want to impose a cross-boundary call to read and write each cell.
- universeのセルをJSに露出するにはいくつかの方法がある
  - はじめに、 `Universe` に `std::fmt::Display` を実装し、セルをテキストとしてレンダーする
  - このRustのStringはWASMのメモリからJSのメモリにコピーされ、JS側でHTMLの`textContent`にセットすることで表示する

Rust Implementation

- `wasm-pack build` でエラー。このバグ踏んだ
  - [wasm-opt: Exported global cannot be mutable · Issue #886 · rustwasm/wasm-pack](https://github.com/rustwasm/wasm-pack/issues/886#issuecomment-667669802)

Rendering to Canvas Directly from Memory

- Rustで `String` を生成し `wasm-bindgen` によってJSの文字列にコンバートさせるのは、不要なセルのコピーを発生させている
- JS側ですでにwidthとheightは知っており、WASMの線形メモリ領域から読むことができるので、ロジックを改善させることができる
- raw wasm module である `wasm_game_of_life_bg` に定義されている `memory` モジュールを通じて、wasmの線形メモリ領域に直接アクセスできる

#### 4.5 Testing Life

> Rust-generated WebAssembly functions cannot return borrowed references.

- `wasm-pack test --chrome --headless` でブラウザを利用したE2Eテストができる

```
Running headless tests in Chrome on `http://127.0.0.1:56291/`
Try find `webdriver.json` for configure browser's capabilities:
Not found
driver status: signal: 9
driver stdout:
    Starting ChromeDriver 74.0.3729.6 (255758eccf3d244491b8a1317aa76e1ce10d57e9-refs/branch-heads/3729@{#29}) on port 56291
    Only local connections are allowed.
    Please protect ports used by ChromeDriver and related test frameworks to prevent access by malicious code.

Error: failed to find element reference in response
error: test failed, to rerun pass '--test web'
Error: Running Wasm tests with wasm-bindgen-test failed
Caused by: failed to execute `cargo test`: exited with exit code: 1
  full command: "cargo" "test" "--target" "wasm32-unknown-unknown"
```

-> Homebrew でインストールした chromedriver のバージョンが古かった

```
$ chromedriver -v
ChromeDriver 74.0.3729.6 (255758eccf3d244491b8a1317aa76e1ce10d57e9-refs/branch-heads/3729@{#29})

$ brew cask upgrade chromedriver
```

https://github.com/rustwasm/wasm-pack/issues/611#issuecomment-522093207
によると、インストール済みの chromedriver を探す？

> `wasm-pack` is searching for currently installed chromedriver (see https://github.com/rustwasm/wasm-pack/blob/master/src/test/webdriver.rs#L34) so we can use that fact.


#### 4.6 Debugging

[Debugging - Rust and WebAssembly](https://rustwasm.github.io/docs/book/reference/debugging.html)

Building with Debug Symbols

- デバッグ時はdebug symbolsを有効にしろ
- debugビルドはデフォルトでsymbolsが有効化されるが、releaseビルドはCargo.toml内で明示的に有効にする必要がある

```toml
[profile.release]
debug = true
```

Logging with the console APIs

- `web-sys` というクレートを使うとRust側でも `console.log` ぽい関数が使える
  - `web_sys::console::log_1(&"Hello, world!".into());`

Logging Panics

- `console_error_panic_hook` クレートを使うとパニック時に `console.error` に表示してくれる

Using a Debugger

- まだまだ未熟(immature)だって書いてある
- [Improved WebAssembly debugging in Chrome DevTools  |  Google Developers](https://developers.google.com/web/updates/2019/12/webassembly) でdevtoolsまわり改善されてる？

Avoid the Need to Debug WebAssembly in the First Place

- バグがJSやWeb APIとのインタラクションに特有のものであれば、 `wasm-bindgen-test` を使う
- そうでなければ、Rustの `#[test]` 関数で再現させる

> Note that in order to run native `#[test]`s without compiler and linker errors, you will need to ensure that "rlib" is included in the `[lib.crate-type]` array in your `Cargo.toml` file.

#### 4.7 Adding Interactivity

- ユーザーがセルをクリックしたら dead/alive をトグルする機能の追加
- wasm に関連して目新しい情報はなし。canvas 上のクリックされた座標をどう変換するかとかの方が勉強

#### 4.8 Time Profiling

[Time Profiling - Rust and WebAssembly](https://rustwasm.github.io/docs/book/reference/time-profiling.html) で紹介されてるもの

- `window.performance.now()`
  - 若干オーバーヘッドがあるので、web-sys crate を使った独自のnow()をRustで実装する方法が紹介されてる。必要？
- devtoolによるプロファイリング
  - debug symbolsを有効にしてるとnameにRustの関数名が表示される
  - NOTE: インライン化された関数は表示されず、RustとLLVMはインライン化にひどく頼っており、結果は少々ややこしいことになる可能性も
- `console.time` と `console.timeEnd`
  - `console.time('foo')` ~ `console.timeEnd('foo')` で囲んだところの実行時間が測定される
  - 結果は console に出力されるだけでなくプロファイラーの"timeline"や"waterfall"ビューにも表示される
- `#[bench]`
  - `#[bench]` をつける、 `benches` ディレクトリにベンチマーク用のファイルを置く
  - NOTE: ネイティブコードのプロファイルに時間を投資する前に、ボトルネックが wasm にあることをブラウザのプロファイラーを使って確認すべき

Creating a Frames Per Second Timer with the `window.performance.now` Function

- `window.performance.now` を使用した計測

Time Each `Universe::tick` with `console.time` and `console.timeEnd`

- `web-sys` で `console.time` と `console.timeEnd` を Rust 側で使って tick 関数の実行時間を計測する
- てっきり関数の最初と最後に time と timeEnd を仕込むのかと思ったけど、構造体を定義して drop 時に timeEnd すればいいというのはなるほどという感じ（よくわかってないけど）

Growing our Game of Life Universe

- devtools のパフォーマンスタブを使ってボトルネック測定する、というJS側だけの話

Making Time Run Faster

- 1回のアニメーションフレームで tick を10回ずつ進めるとまたパフォーマンスが落ちるので、その分析
- Rust の `#[feature(test)]` という機能(nightlyが必要)と、 cargo benchcmp というツールを使う
- さらにOSのパフォーマンス計測コマンドを使った分析方法
  - Linuxだと `perf` というコマンドがあるらしいが、Macにはなかった
  - `fn live_neighbor_count` で剰余算 `%` (div) を使っていたのが原因
- TODO: 実際に試してはいない

#### 4.9 Shrinking .wasm Size

LTO: Link Time Optimizations

- Cargo.toml の `[profile.release]` に `lto = true` とつけるだけ

> This gives LLVM many more opportunities to inline and prune functions. Not only will it make the `.wasm` smaller, but it will also make it faster at runtime! The downside is that compilation will take longer.

コンパイル時間は長くなる以外のデメリットがないので、本番ビルドでは常にONにするのがいいのかな🤔

- `wasm-opt` とは？
  - [Binaryen](https://github.com/WebAssembly/binaryen) に含まれる1ツール、ぽい
  - "Loads WebAssembly and runs Binaryen IR passes on it"
- wasm-pack-template ですでに

    ```toml
    [package.metadata.wasm-pack.profile.release]
    wasm-opt = ["-Oz", "--enable-mutable-globals"]
    ```

    があり、最適化済みなのかどうかよくわからなかった。

#### 4.10 Publishing to npm

npm パッケージ名の衝突を避けるためにCargo.tomlのnameにprefix/suffixつける方法が紹介されてたけど、個人的にはscoped packageにする方が好き

[Packaging and publishing - Hello wasm-pack!](https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/packaging-and-publishing.html)

```sh
$ wasm-pack build --scope MYSCOPE
```
