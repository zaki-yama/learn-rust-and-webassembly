extern "C" {
    fn printNumber(n: i32);
}

#[no_mangle]
fn echo(n: i32) {
    unsafe {
        printNumber(n);
    }
}
