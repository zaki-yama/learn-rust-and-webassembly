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

fn f1(p: &Parent) {
    println!("p: {:?}", p);
}

fn f2(p: &mut Parent) {
    p.0 *= 2;
}

fn main() {
    let mut p1 = Parent(1, Child(11), Child(12));
    f1(&p1);
    f2(&mut p1);

    println!("p1: {:?}", p1);
}
