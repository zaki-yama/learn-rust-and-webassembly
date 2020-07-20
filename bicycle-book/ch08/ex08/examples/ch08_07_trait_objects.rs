use std::fmt::Display;
use std::string::ToString;

fn stringify<T: ToString>(t: T) -> String {
  t.to_string()
}

fn main() {
  stringify(1i32);

  let mut v: Vec<&dyn Display> = vec![];
  v.push(&true);
  v.push(&1i32);
}
