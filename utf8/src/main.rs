fn main() {
    // 1
    let s = "whatever".to_string();
    // 2
    let s = String::from("whatever");
    // 3
    let mut s = String::from("foo");

    s.push_str("bar");
    s.push('!');

    println!("the value of s is {}", s);
}
