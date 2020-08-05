#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

extern "C" {
    fn date_now() -> f64;
}

#[no_mangle]
pub fn get_timestamp() -> f64 {
    unsafe { date_now() }
}
