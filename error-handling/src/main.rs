// fn divide(numerator: f64, denominator: f64) -> Option<f64> {
//     if denominator == 0.0 {
//         None
//     } else {
//         Some(numerator / denominator)
//     }
// }

fn divideV2(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        return Err("Cannot divide by 0".to_string());
    } else {
        return Ok(numerator / denominator);
    }
}

// 2

fn main() {
    // println!("Hello, world!");

    // let result = divide(10.0, 2.0);
    // let resultV2 = divideV2(100.23, 73.89);

    // match result {
    //     Some(x) => println!("Result: {}", x),
    //     None => println!("Cannot divide by Zero!"),
    // }

    match divideV2(100.23, 1.0) {
        Ok(resultV2) => println!("Result: {}", resultV2),
        Err(err) => println!("Error: {}", err),
    };
}
