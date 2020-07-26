hello-wasm in MDN
=================

- [ ]  [WebAssembly | MDN](https://developer.mozilla.org/ja/docs/WebAssembly)
- [x]  [WebAssembly の概要 - WebAssembly | MDN](https://developer.mozilla.org/ja/docs/WebAssembly/Concepts)
- [ ]  [WebAssemblyコードのロードと実行 - WebAssembly | MDN](https://developer.mozilla.org/ja/docs/WebAssembly/Loading_and_running)
- [ ]  [WebAssembly JavaScript API を使用する - WebAssembly | MDN](https://developer.mozilla.org/ja/docs/WebAssembly/Using_the_JavaScript_API)
- [ ]  [エクスポートされた WebAssembly 関数 - WebAssembly | MDN](https://developer.mozilla.org/ja/docs/WebAssembly/Exported_functions)
- [x]  [Rust から WebAssembly にコンパイルする - WebAssembly | MDN](https://developer.mozilla.org/ja/docs/WebAssembly/rust_to_wasm)



### Rust 側

- `extern` の中に、「Rust 内で使いたい JS の関数のシグネチャ」を書く
- 「JS 側で使いたい Rust で書いた関数」は `pub fn xxx` で宣言
- ↑どちらも `#[wasm_bindgen]` が必要


ビルド

```
$ wasm-pack build --scope zaki-yama

$ tree pkg
pkg
├── hello_wasm.d.ts
├── hello_wasm.js
├── hello_wasm_bg.d.ts
├── hello_wasm_bg.js
├── hello_wasm_bg.wasm
└── package.json
```

### トラブルシューティング

- 英語版は `npm publish` せず `npm link` で行っている
  - https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm#Making_our_package_availabe_to_npm
- そのままだと `Can't resolve './hello_wasm_bg.js'` エラー
  - たぶんこれのせい: [Build doesn't include all required files · Issue #881 · rustwasm/wasm-pack](https://github.com/rustwasm/wasm-pack/issues/881)
