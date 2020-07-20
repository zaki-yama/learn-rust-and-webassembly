fn to_n(n: i32) -> impl Iterator {
  0..n
}

use std::iter::Filter;
use std::ops::Range;

// 複雑なイテレータになると型を書くのも大変
// fn to_n_even(n: i32) -> Filter<Range<i32>, fn(&i32) -> bool> {
fn to_n_even(n: i32) -> impl Iterator {
  (0..n).filter(|i| i % 2 == 0)
}

// 存在impl Traitで表現された値の使用側では元の型の情報が失われる
use std::fmt;
fn one() -> impl fmt::Display {
  1i32
}

fn main() {
  let n = one();
  // println!("{}", n + n); // error
}
