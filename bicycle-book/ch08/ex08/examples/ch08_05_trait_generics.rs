trait As<T> {
  fn cast(self) -> T;
}

// 実装をジェネリックにせずに個別の型に対して実装する
impl As<u64> for u8 {
  fn cast(self) -> u64 {
    self as u64
  }
}

impl As<u32> for u8 {
  fn cast(self) -> u32 {
    self as u32
  }
}

fn main() {
  // トレイと実装で指定した型はcastに指定できる
  let one_u32: u32 = 1.cast();
  let one_u32: u64 = 1.cast();
  // i8は指定してないのでこれはエラー
  // let one_u32: i8 = 1.cast();
}
