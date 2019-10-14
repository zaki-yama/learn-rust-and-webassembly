// 可変な参照
// fn main() {
//     let mut s = String::from("hello");

//     // change(&mut s);

//     // println!("{}", s);

// }
// 
// fn change(s: &mut String) {
//     s.push_str(",world");
// }

// 可変な参照は特定のスコープ内では一つまで
// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s;
// 
//     // error
//     println!("{}, {}", r1, r2);
// }

// ダングリング参照
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

