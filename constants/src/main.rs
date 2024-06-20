fn main() {
    println!("Hello, world!");
    const Y: i32 = 10; // variable name must be in capital letter and must be declared type
    println!("The value of Y is: {}", Y);

    println!("The valure of PI is: {}", PI);

    println!(
        "The value of THREE_HOURS_IN_SECONDS is: {}",
        THREE_HOURS_IN_SECONDS
    );
}

// can declare the constant here (outside the function)
const PI: f64 = 3.141592653;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
