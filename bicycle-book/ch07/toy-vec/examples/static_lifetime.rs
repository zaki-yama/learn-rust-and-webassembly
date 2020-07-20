fn take_static<T: 'static>(_x: T) {}

fn main() {
  static I0: i32 = 42;

  let mut s0: &'static str;
  let s1 = "42"; // &str型。文字列リテラルへの参照（データは静的領域にある）
  let s2 = 42.to_string(); // String型（データはヒープ領域にある）

  s0 = s1;
  s0 = &s2; // コンパイルエラー。String型から&'static strは作れない

  take_static(s1); // &'static str型。OK
  take_static(&s2); // &String型。'static要求を満たせないのでコンパイルエラー
  take_static(s2); // String型。OK
}
