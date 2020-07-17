// アイテムのインポートも、もちろん必要
use std::io::Cursor;
use wordcount::{count, CountOption};

#[macro_use]
mod utils;

#[test]
fn char_count_works() {
  let input = Cursor::new(b"abadracadabra");

  let freq = count(input, CountOption::Char);
  assert_map!(freq, {
      "a" => 6,
      "b" => 2,
      "c" => 1,
      "d" => 2,
      "r" => 2
    }
  );
}
