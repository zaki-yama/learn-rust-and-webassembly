use toy_vec::ToyVec;

fn main() {
  let e: Option<&String>;
  {
    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());
    v.push("Budgerigar".to_string());
    e = v.get(1);
  }

  assert_eq!(e, Some(&"Budgerigar".to_string()));
}
