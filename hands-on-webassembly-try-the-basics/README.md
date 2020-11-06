Hands-on WebAssembly: Try the basics
====================================


[Hands-on WebAssembly: Try the basics — Martian Chronicles, Evil Martians’ team blog](https://evilmartians.com/chronicles/hands-on-webassembly-try-the-basics)  
翻訳：[WebAssemblyハンズオン: 実際に動かして基礎を学ぶ（翻訳）｜TechRacho（テックラッチョ）〜エンジニアの「？」を「！」に〜｜BPS株式会社](https://techracho.bpsinc.jp/hachi8833/2020_11_02/97774)

## 使用したコマンド

NOTE: すべて

```zsh
$ docker run --rm -v $(pwd):$(pwd) -w $(pwd) zloymult/wasm-build-kit \
```

の後に続ける。

```zsh
# コンパイル(C)
$ clang --target=wasm32 -O3 -nostdlib -Wl,--no-entry -Wl,--export-all -o dragon-curve.wasm dragon-curve.c

# `.wasm` の中身を確認
$ wasm-objdump dragon-curve.wasm -s

# Bynarien を使った最適化
$ wasm-opt -Os dragon-curve.wasm -o dragon-curve-opt.wasm

# WebAssembly Binary Toolkit (wabt) を使って .wasm -> .wat 変換
$ wasm2wat dragon-curve-opt.wasm > dragon-curve-opt.wat

# emscripten
$ emcc dragon-curve.c -Os -o dragon-curve.js \
-s EXPORTED_FUNCTIONS='["_dragonCurve", "_malloc", "_free"]' \
-s EXPORTED_RUNTIME_METHODS='["ccall"]' \
-s MODULARIZE=1
```

