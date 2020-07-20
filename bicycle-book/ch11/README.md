
第11章 Webアプリケーション、データベース接続
======================================

- RustのWeb開発向けの機能は発展途上。非同期IOなど
- Hyper: デファクトスタンダードになっているHTTPライブラリ。0.10系までは同期IOだが、0.11系以降は非同期IO
- FuturesとTokio
  - Futures: 並行デザインパターンのFutureをRust上に実装したもの。Promiseのことらしい
  - Tokio: イベント駆動なネットワークの非同期IOやタスクの非同期実行などをFuturesを使ってラップしたもの
- async/await は Future の糖衣構文
- 参考
  - Async Book https://rust-lang.github.io/async-book/
  - [Are we async yet?](https://areweasyncyet.rs/)
- Web アプリケーションフレームワーク
  - ref. [Choosing a Rust web framework, 2020 edition](https://www.lpalmieri.com/posts/2020-07-04-choosing-a-rust-web-framework-2020-edition/)
    - actix-web
    - rocket
    - tide
    - warp

### 11-2 Web アプリケーションフレームワーク Actix Web

- Hyperを使わずアクターフレームワークの上に作られている
- アクター: 非同期にメッセージをやりとりする存在。独立性が高い

11-2-2

- 構成要素
  - HttpServer
  - App
  - ハンドラ
  - エクストラクタ
  - ミドルウェア

ハンドラ

```rust
pub trait Handler<S>:L 'static {
  type Result: Responder;
  fn handle(&self, req: &HttpRequest<S>) -> Self::Result;
}
```

Responder

```rust
pub trait Responder {
  type Item: Into<AsyncResult<HttpResponse>>;
  type Error: Into<Error>;
  fn respond_to<S: 'static>(
    self,
    req: &HttpRequest<S>
  ) -> Result<Self::Item, Self::Error>;
}
```

おおざっぱな流れ

- ハンドラを呼ぶ。ハンドラの戻り値はString
- ReponderがStringを受け取ってHttpResponseを作る (repond_to)
- Into を使って非同期の結果として扱う
