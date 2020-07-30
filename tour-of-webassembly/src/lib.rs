#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

extern "C" {
    fn get_magic_number() -> i32;
    fn wasm_log(start: usize, len: usize);
}

#[no_mangle]
pub fn main() {
    log("hello world!");
}

// 14 テキストのロギング
fn log(msg: &str) {
    let start = msg.as_ptr();
    let len = msg.len();
    unsafe {
        wasm_log(start as usize, len);
    }
}
