extern crate tinymt;

use tinymt::tinymt32;

#[no_mangle]
pub fn rand() -> u32 {
  let param = tinymt32::Param {
    mat1: 0x8F70_11EE,
    mat2: 0xFC78_FF1F,
    tmat: 0x3793_fdff,
  };
  let seed = 1;
  tinymt32::from_seed(param, seed).gen()
}

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
  a + b
}
