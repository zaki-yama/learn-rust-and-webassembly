use std::ops::{Deref, Drop};
struct TattleTell<T> {
    value: T,
}
impl<T> Deref for TattleTell<T> {
    type Target = T;
    fn deref(&self) -> &T {
        println!("{} was used!", std::any::type_name::<T>());
        &self.value
    }
}

impl<T> Drop for TattleTell<T> {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

fn main() {
    let foo = TattleTell {
        value: "secret message",
    };
    // dereference occurs here immediately
    // after foo is auto-referenced for the
    // function `len`
    println!("{}", foo.len());
}
