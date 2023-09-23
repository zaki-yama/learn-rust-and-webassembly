fn main() {
  // 型を明示的に指定
  let mut i32_vec = Vec::<i32>::new(); // turbofish <3
  i32_vec.push(1);
  i32_vec.push(2);
  i32_vec.push(3);

  // もっと賢く、型を自動的に推論
  let mut float_vec = Vec::new();
  float_vec.push(1.3);
  float_vec.push(2.3);
  float_vec.push(3.4);

  // きれいなマクロ！
  let string_vec = vec![String::from("Hello"), String::from("World")];

  for word in string_vec.iter() {
      println!("{}", word);
  }
}
