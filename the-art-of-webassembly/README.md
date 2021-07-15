## 疑問

- stack と local variable や global variable が格納される領域って違うの？
- ch06
  - `(global $obj_base_addr (import "env" "obj_base_addr") i32)`
    - これで import した値がセットされるんだっけ？ global 変数の基本的な構文の話
- ch07
  - `instantiate` と `instantiateStreaming` の違い (Node.js は前者しか使えない)
- ch08
  - `i32.load` と `i32.store` ってどういう動きになるんだっけ？

## 学び

### Chapter 2 WebAssembly Text Basics

- importObject の第一階層、`env` である必要ないんだ
  - Rust で書く場合はどうかな
- WAT は S 式 と linear instruction set の 2 つの書き方がある。混在もできる

### Chapter 3 Functions And Tables
