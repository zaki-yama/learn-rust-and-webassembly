fn main() {
    let a = 42;
    let memory_location = &a as *const i32 as usize;
    println!("Data is here {}", memory_location);
}
