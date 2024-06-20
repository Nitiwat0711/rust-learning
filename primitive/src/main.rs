fn main() {
    // int, float, bool, char
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;

    println!("Maximum Value of i32: {}", e);
    println!("Maximum Value of i64: {}", i);

    // Floats
    // f32, f64

    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    // Boolean
    let is_snowing: bool = true;
    println!("Is it snowing: {}", is_snowing);

    // Char - char

    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);
}
