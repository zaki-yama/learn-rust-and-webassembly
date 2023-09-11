fn main() {
    let helloworld = vec!["hello", " ", "world", "!"].concat();
    let helloworld = ["hello", " ", "world", "!"].concat();
    let abc = ["a", "b", "c"].join(",");
    println!("{}", helloworld);
    println!("{}", abc);
}
