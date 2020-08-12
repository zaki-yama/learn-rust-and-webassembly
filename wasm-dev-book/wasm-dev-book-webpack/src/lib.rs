extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "./index")]
extern "C" {
    fn date_now() -> f64;
}

#[wasm_bindgen]
pub fn get_timestamp() -> f64 {
    date_now()
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
