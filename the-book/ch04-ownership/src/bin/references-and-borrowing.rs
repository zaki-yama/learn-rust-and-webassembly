fn main() {
    let s1 = String::from("hello");

    let len = calculate_len(&s1);

    println!("The length of {} is {}", s1, len);

    change(&s1);
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

fn change(some_string: &String) {
    // cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    // some_string.push_str(", world");
}
