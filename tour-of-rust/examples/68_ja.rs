fn say_it_loud(msg: &str) {
    println!("{}!!!", msg.to_string().to_uppercase());
}

fn main() {
    // say_it_loudは&'static strを&strとして借用することができます
    say_it_loud("hello");
    // say_it_loudはStringを&strとして借用することもできます
    say_it_loud(&String::from("goodbye"));
}
