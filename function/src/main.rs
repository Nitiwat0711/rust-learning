fn main() {
    hello_world();
    tell_height(177);
    human_id("Nitiwat", 24, 177.0);

    let _x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };
    println!("Result is: {}", _x);

    let y = add(4, 6);
    println!("Value of y is : {}", y);
    println!("Value from function 'add' is: {}.", add(4,6));

    let weight: f64 = 63.5;
    let height: f64 = 1.77;
    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi);
}

fn hello_world() {
    println!("Hello, Rust!");
}

fn tell_height(height: u32) {
    println!("My height is {} cm", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {}, I am {} years old, and my height is {} cm.", name, age, height);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}

