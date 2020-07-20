trait Init<T> {
  fn init(t: T) -> Self;
}

impl<T> Init<T> for Box<T> {
  fn init(t: T) -> Self {
    Box::new(t)
  }
}

fn main() {
  // ジェネリクスが推論可能なら省略できる
  let data = Box::init("foo");

  // トレイトのジェネリクス型を明治するには型名::<型>と書く
  let data = Box::<f32>::init(0.1);
  // 文脈から型が推論できる場合はInitとトレイト名でも書ける
  let data: Box<f32> = Init::init(0.1);
  let data: Box<_> = Init::<f32>::init(0.1);
}
