// fn main() {
//     // - each value in Rust has an owner
//     let s1 = String::from("RUST");
//     let len = calculate_length(&s1);
//     println!("Length of '{}' is {}.", s1, len);

//     // - there can only be one owner at the time
//     let s2 = s1;

//     // println!("{}", s1);
//     println!("{}", s2);

// }

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Ownership
// - each value in Rust has an owner
// - there can only be one owner at the time


// - when the owner goes out of scope, the value wiil be dropped

fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);

    println!("Length of '{}' is {}.", s1, len);
}

fn printLost(s: &String) {
    println!("{}", &s1);
}

