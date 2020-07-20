use toy_vec::ToyVec;

fn main() {
  let mut v = ToyVec::new();
  v.push("Java Finch".to_string());
  v.push("Budgerigar".to_string());

  let mut iter = v.iter();

  // v.push("Hill Mynah".to_string()); // error
  // pushは可変の参照を得ようとするが、iterが生存しているので不変の参照が有効

  assert_eq!(iter.next(), Some(&"Java Finch".to_string()));
  v.push("Canary".to_string());
}
