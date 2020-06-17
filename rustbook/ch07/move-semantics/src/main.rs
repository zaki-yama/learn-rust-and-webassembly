// 7-5 ムーブセマンティクス

#[derive(Debug)]
struct Parent(usize, Child, Child);

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

#[derive(Debug)]
struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    let mut p1 = Parent(1, Child(11), Child(12));
    let p2 = p1; // 値の所有権をp1からp2にムーブする
    println!("p2: {:?}", p2);
    // println!("p1: {:?}", p1); // error

    p1 = Parent(2, Child(21), Child(22));
    // p1は別の値の所有権を持つためアクセスできる
    println!("p1: {:?}", p1);
}
