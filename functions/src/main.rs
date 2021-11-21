fn main() {
    println!("Hello, function!");
    another_function();
    with_argument(5, 'h');
    println!("Implicit returned: {}", implicit_return(55));
    println!("Explicit returned: {}", explicit_return(55));
}

fn another_function() {
    println!("From another function!");
}

fn with_argument(num: i32, time: char) {
    println!("This course takes {}{}.", num, time);
}

// return with an expression
fn implicit_return(num: u32) -> u32 {
    num
}

// return with a return statement
fn explicit_return(num: i32) -> i32 {
    return num;
}