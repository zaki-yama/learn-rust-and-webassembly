/// 2.1.5 参照型とポインタ型
fn ch2_1_5() {
    let mut n: u64 = 100;

    let a: &u64 = &n;
    // *a = 200; 破壊的代入はできない
    println!("*a: {}, addr = {:p}", *a, a);

    let b: &mut u64 = &mut n;
    *b = 200;
    println!("n = {}", n);
}

fn main() {
    ch2_1_5();
}
