fn main() {
    let s = String::from("hello");  // sがスコープに入る

    takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない

    let x = gives_ownership();

    println!("{}", x);
    // println!("{}", s);  // error
} // ここでs,xがスコープを抜ける。xはドロップされるが、sの値はムーブされてるので、何も特別なことはない。

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  // 

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string  // 呼び出し元関数にsome_stringがムーブされる
}
