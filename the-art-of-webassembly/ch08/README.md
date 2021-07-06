# Chapter 8 Working with the Canvas

- wasmbook 的な内容ぽい
- Chapter 6 でやった衝突判定プログラムを再利用

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
