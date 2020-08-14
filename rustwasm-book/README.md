Rust 🦀 and WebAssembly 🕸
===========================

https://rustwasm.github.io/docs/book/introduction.html

## 2. Why Rust and WebAssembly?

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
