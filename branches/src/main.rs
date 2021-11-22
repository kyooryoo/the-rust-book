fn main() {
    println!("Hello, branches!");

    // a sample if expression
    let x = 4;
    let y = 5;
    let z = if x > y {
        println!("{} is greater than {}!", x, y);
    } else if x < y {
        println!("{} is less than {}!", x, y);
    } else {
        println!("{} is equal {}!", x, y);
    };
    // if nothing returns, if returns unit value ()
    println!("{:?}", z);

    // ONLY boolean could be used for conditions
    let num = 3;
    // if num {
    if num != 0 {
        println!("{} is not equal to 0!", num);
    }

    // use if in a let statement
    let weekday_key = 5;
    let weekday = if weekday_key == 5 {
        "Friday!"
    } else {
        "Anyday."
    };
    println!("Today is {}", weekday);
}
