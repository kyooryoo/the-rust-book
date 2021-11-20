#![allow(unused)]
fn main() {
    println!("Hello, variables!");

    // use keyword mut if value may change
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // define constant with uppercase name
    // specifying data type is necessary
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // define the same variable for shadowing the first one
    let x = 5;
    let x = x + 1;
    {
        // the seond shadow only lives in the inner scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    // in the outer scope, x still has value 6
    println!("The value of x is: {}", x);

    // when shadowing, data type could change. 
    let spaces = "   "; // string type
    let spaces = spaces.len(); // number type    
}
