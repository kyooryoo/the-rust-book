// import the library that is not imported defaultly
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // the following line of code is for testing purpose
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // keyword mut for variable that changes value
        // Use new() method for creating str variable
        let mut guess = String::new();

        // Use stdin() method from std module for input
        io::stdin()
            // since reference is immutable as well
            // instead of "&guess" we use "&mut guess"
            .read_line(&mut guess)
            // read_line may return result of OK or ERR
            // especially, ERR should be handled as follows
            .expect("Failed to read line");

        // parse method returns an enum with Ok or Err
        let guess: u32 = match guess.trim().parse() {
            Result::Ok(num) => num,
            Result::Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
