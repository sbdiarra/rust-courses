mod temp_converter;

use std::borrow::Borrow;

// #![allow(unused)]
fn main() {
    // variables
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);

    // constants
    const MAX_POINTS: u32 = 100_000;

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tupDestructirng = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);


    // array
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let arr = [1, 2, 3, 4, 5];


    let s1 = String::from("hello");
    let s2 = s1;
}

