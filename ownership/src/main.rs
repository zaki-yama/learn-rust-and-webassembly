fn main() {
    let s = "hello";
    println!("Hello, world!");
    
    // move
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // error

    // clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
