fn main() {
    println!("Hello, function!");
    
    another_function();
    with_argument(5, 'h');
    println!("Implicit returned: {}", implicit_return(55));
    println!("Explicit returned: {}", explicit_return(55));

    let a: u64 = 17;
    let b = 3;
    let result = add(a, b);
    println!("adding {} and {} gets {}", a, b, result);

    let score = 2048;
    increase_by(score, 30);
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

fn add(a: u64, b: u64) -> u64 {
    a + b
}

// change passed in value
fn increase_by(mut val: u32, how_much: u32) {
    val += how_much;
    println!("You made {} points!", val);
}