# 📕 The Art of WebAssembly

https://nostarch.com/art-webassembly

## 疑問

- stack と local variable や global variable が格納される領域って違うの？
- ch06
  - `(global $obj_base_addr (import "env" "obj_base_addr") i32)`
    - これで import した値がセットされるんだっけ？ global 変数の基本的な構文の話
- ch07
  - `instantiate` と `instantiateStreaming` の違い (Node.js は前者しか使えない)
- ch08
  - ✅ `i32.load` と `i32.store` ってどういう動きになるんだっけ？ -> linear memory の値の読み込みとセット
