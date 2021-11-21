#![allow(unused)]
use std::io;

fn main() {
    println!("Hello, world!");

    // demo overflow handling

    let x: u8 = 255;
    let y: u8 = 1;

    // overflow will cause panic
    // let z: u8 = 255 + 1;

    // explicit wrapping
    let z = x.wrapping_add(y);
    assert_eq!(z, 0);

    // checked method
    let z = x.checked_add(y);
    assert_eq!(z, None);

    // overflowing method
    let z = x.overflowing_add(y);
    // println!("{:?}", z);
    assert_eq!(z.0, 0);
    assert_eq!(z.1, true);

    let z = x.saturating_add(y);
    assert_eq!(z, 255);

    // floating numbers
    let x = 2.0; // default is f64
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    // boolean type
    let t = 3 > 2;
    println!("3 > 2: {}", t);
    // explicit type annotation
    let f: bool = 3 < 2;
    println!("3 < 2: {}", f);

    // Unicode characters
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("char supported: {} {} {}", c, z, heart_eyed_cat);

    // tuple has fixed length
    let tup: (i32, u64, f32, char) = (3, 3, 3.3, '3');
    // use pattern matching to desctructure a tuple
    let (a, b, c, d) = tup;
    // access tuple element by index
    println!("tup has {} amd {}", c, tup.3);

    // array has fixed length as well
    // and all members are in the same type
    // declare array type and number of members
    let mut a:[i32;5] = [1,2,3,4,5];
    // we can change array member value only if:
    // we define the array as mutable in advance
    a[0] = 100;
    println!("The a[0] now changed to: {}", a[0]);
    // or declare member value and their number
    let b = [3;5];
    
    // demo a runtime error
    println!("Please enter array index between 0 and 4:");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
