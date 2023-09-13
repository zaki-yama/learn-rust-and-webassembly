struct Pie;

impl Pie {
    fn eat(&self) {
        println!("tastes better on the heap!")
    }
}

fn main() {
    let heap_pie = Box::new(Pie);
    heap_pie.eat();
}
