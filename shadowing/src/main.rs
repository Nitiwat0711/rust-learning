fn main() {
    println!("Hello, world!");

    let x = 5;
    let x = x + 1;
    let x = x * 1000;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in the main function is: {x}");
    // can declare variable with the same name as a previous variable
    // not equals mut
}
