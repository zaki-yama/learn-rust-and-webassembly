use std::str::FromStr;

trait Server {
  // type 型名で関連型を宣言できる
  type Response;

  // あるいはtype 型名:トレイト境界 で境界を設定することもできる
  type Request: FromStr;

  // 関連型にはSelf::型名でアクセスする
  fn handle(&self, req: Self::Request) -> Self::Response;
}

struct EchoServer;

impl Server for EchoServer {
  // トップレベルと同じようにtype 型名 = 型名で定義できる
  type Response = String;
  // トレイト境界のついた型も同じように定義できる
  // トレイト境界を満たさない型を書くとコンパイルエラーになる
  type Request = String;

  fn handle(&self, req: Self::Request) -> Self::Response {
    req
  }
}

// S::ResponseのようにServerの関連型を参照できる
// 関連型については特別指定しなければ任意の関連型を受け付ける
// fn handle<S: Server>(server: S, req: &str) -> S::Response {
//   let req = S::Request::from_str(&req).unwrap();
//   server.handle(req)
// }

// あるいは、関連型が特定の型を持っていることを指定したければ、
// トレイト名<関連型名 = 型>のように指定できる
// この場合RequestにStringを持つServerの実装しか受け付けない
fn handle<S: Server<Request = String>>(server: S, req: &str) -> S::Response {
  server.handle(req.to_string())
}

fn main() {
  let req = String::from("request");
  let res = String::from("response");

  let server = EchoServer;
  println!("{}", handle(server, &req));
}
