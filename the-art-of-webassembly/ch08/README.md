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
