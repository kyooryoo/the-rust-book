use std::io;
use substring::Substring;

fn main() {
    println!("Hello, loop!");

    // demo the embedded loops
    let mut count = 0;
    // label the outside loop with counting_up
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                // this breaks the inner loop
                break;
            }
            if count == 2 {
                // this breaks the outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // demo the returned value with break
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // demo the while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // demo loop through a collection
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;
    while index < 5 {
        println!("the value from while is: {}", a[index]);
        index += 1;
    }
    // for works with better safety and efficiency
    for element in a {
        println!("the value from for is: {}", element);
    }
    // use for instead of while to control looping times
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // convert between Fahrenheit and Celsius temperatures
    println!("Input Fahrenheit or Celsius temperature as FXX or CXX:");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Cannot read line!");
    let kind = temp.chars().nth(0).unwrap();

    let value: f64 = temp
        .substring(1, temp.len() - 1)
        .parse()
        .expect("Temperature value should be a number");

    if kind == 'F' {
        let result = (value - 32.0) / 1.8;
        println!("Fehrenheit {} in Celsius is: {}", value, result);
    } else if kind == 'C' {
        let result = value * 1.8 + 32.0;
        println!("Celsius {} in Fehrenheit is: {}", value, result);
    } else {
        println!("Temperature type is invalid!");
    }

    // generate the nth Fibonacci number
    println!("Input a number > 3 for getting nth Fibonacci number:");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Cannot read line!");
    let n: u64 = n.trim().parse().expect("Input a number!");
    let mut result: u64 = 0;
    if n > 3 {
        let mut num1 = 1;
        let mut num2 = 1;
        result = num1 + num2;
        for _i in 3..n {
            num1 = num2;
            num2 = result;
            result = num1 + num2;
        }
    } else {
        println! {"The number must > 3!"};
    }
    println!("The {}th Fibonacci number is: {}\n", n, result);

    // lyrics of the twelve days of christmas
    let sentences = [
        "Twelve drummers drummin'",
        "Eleven pipers pipin'",
        "Ten lords a leapin'",
        "Nine ladies dancin'",
        "Eight maids milkin'",
        "Seven swans a swimmin'",
        "Six geese a layin'",
        "Five golden rings!",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves",
        "And a partridge in a pear tree!",
    ];

    let numbers = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];

    for i in 0..12 {
        if i > 0 {
            println!("\n{}", sentences[11-i]);
        }
        println!("On the {} day of Christmas", numbers[i]);
        println!("My true love sent to me:");
        for j in (0..i+1).rev() {
            if i == 0 {
                println!("{}", "A partridge in a pear tree!");
            } else {
                println!("{}", sentences[11-j]);
            }
        }
    }
}
