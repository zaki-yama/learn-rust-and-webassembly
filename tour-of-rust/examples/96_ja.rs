fn main() {
    let a: [u8; 4] = [86, 14, 73, 64];
    // this is a raw pointer. Getting the memory address
    // of something as a number is totally safe
    let pointer_a = &a as *const u8 as usize;
    println!("Data memory location: {}", pointer_a);
    // Turning our number into a raw pointer to a f32 is
    // also safe to do.
    let pointer_b = pointer_a as *const f32;
    let b = unsafe {
        // This is unsafe because we are telling the compiler
        // to assume our pointer is a valid f32 and
        // dereference it's value into the variable b.
        // Rust has no way to verify this assumption is true.
        *pointer_b
    };
    println!("I swear this is a pie! {}", b);
}
