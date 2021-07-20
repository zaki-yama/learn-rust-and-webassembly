# Chapter 8 Working with the Canvas

- wasmbook 的な内容ぽい
- Chapter 6 でやった衝突判定プログラムを再利用

<!-- TOC -->

- [Chapter 8 Working with the Canvas](#chapter-8-working-with-the-canvas)
  - [Rendering to the Canvas](#rendering-to-the-canvas)
    - [Defining JavaScript Constants in HTML](#defining-javascript-constants-in-html)
    - [Creating Random Objects](#creating-random-objects)
    - [Bitmap Image Data](#bitmap-image-data)
    - [The requestAnimationFrame Function](#the-requestanimationframe-function)
  - [The WAT Module](#the-wat-module)
    - [Imported Values](#imported-values)
    - [Clearing the Canvas](#clearing-the-canvas)
    - [Absolute Value Function](#absolute-value-function)
    - [Setting a Pixel Color](#setting-a-pixel-color)
    - [Drawing the Object](#drawing-the-object)
    - [Setting and Getting Object Attributes](#setting-and-getting-object-attributes)
    - [The $main Function](#the-main-function)

<!-- /TOC -->

## Rendering to the Canvas

- DOM 同様、WebAssembly は Canvas と直接やりとりできない。  
  かわりに、ピクセルデータを linear memory から直接 canvas 要素に描画する必要がある

### Defining JavaScript Constants in HTML

- JS と Wasm の間で configuration のための定数を共有する
- 定数は
  - canvas-related constants
  - データの構成に関する constants: base address, stride, offsets
  - `ImageData` オブジェクト

### Creating Random Objects

### Bitmap Image Data

- bitmap image を canvas に描画するには `putImageData()` というメソッドが使える

### The requestAnimationFrame Function

- "when you call `requestAnimationFrame`, the function passed to `requestAnimationFrame` is called the next time a frame is rendered. To `requestAnimationFrame`, we pass the function we want to call the next time our computer is ready to render a frame of animation."

## The WAT Module

### Imported Values

### Clearing the Canvas

- フレームごとに canvas 全体の描画をクリアするための関数 `clear_canvas` を作る

### Absolute Value Function

- Chapter 6 では円の衝突判定だったが、ここでは箱(box)の衝突判定
- ここでは、その衝突判定に必要となる絶対値取得関数 `$abs` を作る

### Setting a Pixel Color

- "That function will need a bounds check because we’re writing to an area of linear memory set aside to represent the area of the canvas. Without this check, if we try to write to a memory location that isn’t on the canvas, the function will be writing to an area of linear memory that we might be using for some other purpose."
  - canvas の境界からはみ出てないかチェックする必要がある。なぜなら canvas の領域を表す linear memory の領域に書き込まないと、他の目的で使用してる linear memory の領域に書き込んでしまうかもしれないので
  - wasmbook でも似たようなことやってたはず
- 2 次元の canvas の座標と linear memory の index との対応関係。図解でわかりやすい
  - index = y \* (x 方向のピクセル数) + x

### Drawing the Object

- 正方形のオブジェクトを描画する処理

### Setting and Getting Object Attributes

- atribute を get/set するヘルパーの実装
- attribute の get と set でアドレスの計算は共通化できる (DRY) けど、しばしばパフォーマンス低下につながることがあるのでやらない
  - また、この後の Chapter でさらに DRY をやめることになる
- 他のアセンブリ言語においては、マクロは DRY 原則とパフォーマンスを両立させるためのすばらしい方法だが、wat2wasm は現在マクロをサポートしていない

### The $main Function

- 毎フレーム JS から呼ばれる処理
- オブジェクトを velocity に基づいて動かし、衝突判定し、色を塗る
- Defining Local Variables
- The $move_loop
  - フレームごとにオブジェクトを移動させる処理
- Beginning of the Outer Loop, The Inner Loop
  - 衝突判定の 2 重ループ
  - 2 つのオブジェクトの x 座標の差 < オブジェクトのサイズ && 2 つのオブジェクトの y 座標の差 < オブジェクト なら衝突していることになる
- Redrawing the Objects
  - 再計算された座標と衝突フラグを元に再描画
