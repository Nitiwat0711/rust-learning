fn main() {
    println!("Hello, world!");
    // let _v: Vec<i32> = Vec::new();

    // let mut _v: Vec<i32> = Vec::new();
    // let mut _v: Vec<i32> = vec![1, 2, 3];
    // _v.push(5);
    // _v.push(6);
    // _v.push(7);
    // _v.push(8);
    // _v.push(9);

    // println!("{:?}", _v);

    let _v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &_v[2]; // reference

    println!("The third element is: {third}");

    let third = _v.get(2);

    match third {
        Some(third) => println!("The third element is: {third}"),
        None => println!("There is no third element"),
    }
}
