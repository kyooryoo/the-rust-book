#![allow(unused)]
use std::io;

fn main() {
    println!("Hello, data type!");

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

    println!("\nDEMO of Tuples:");
    // tuple has fixed length but can mix data types
    let tup: (i32, u64, f32, char) = (3, 3, 3.3, '3');
    // use pattern matching to desctructure a tuple
    let (a, b, c, d) = tup;
    // access tuple element by index
    println!("tup has {} amd {}", c, tup.3);

    println!("\nDEMO of Arrays:");
    // array has fixed length as well
    // and all members are in the same type
    // declare array type and number of members
    let mut a:[i32;5] = [1,2,3,4,5];
    // we can change array member value only if:
    // we define the array as mutable in advance
    a[0] = 100;
    println!("The a[0] now changed to: {}", a[0]);
    // or declare member value and their number
    // following code define an array with 5 3s
    let b = [3;5];
    println!("Another array: {:?}", b);
    // or list of items with optional type on first item
    let c = [0.1, 0.2, 0.3];
    let d = [0.1f64, 0.2, 0.3];
    println!("{:?} {:?}", c, d);
    
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

    let question = "How are you?"; // &str type
    let person: String = "Bob".to_string();
    let youkoso = String::from("ようこそ"); // unicode
    println!("{}! {} {}", youkoso, question, person);

    let value = Dummy;

    // demo tuple struct
    let white = Color(255, 255, 255);
    let red = white.0;
    let green = white.1;
    let blue = white.2;
    println!("Red value: {}", red);
    println!("Green value: {}", green);
    println!("Blue value: {}", blue);

    let orange = Color(255, 165, 0);
    let Color(r, g, b) = orange;
    println!("R: {}, G: {}, B: {} (ornage)", r, g, b);
    let Color(r, _, b) = orange;

    // demo C lang struct
    let name = "Alice".to_string();
    let player = Player {
        name,
        iq: 171,
        friends: 134,
        score: 1129
    };
    bump_player_score(player, 120);

    // demo enum
    let simulated_player_action = PlayerAction::Move {
        direction: Direction::N,
        speed: 2,
    };
    match simulated_player_action {
        PlayerAction::Wait => println!("Player wants to wait."),
        PlayerAction::Move { direction, speed } => {
            println!("Player wants to move in direction {:?} with speed {}.", direction, speed)
        },
        PlayerAction::Attack(direction) => {
            println!("Player wants to attack direction {:?}.", direction)
        }
    }

    // demo impl of struct
    let mut player = Player::with_name("Dave");
    player.set_friends(23); // instance.method
    println!("{}'s friends count: {}", player.name, player.get_friends());
    // another way to call instance method
    let friends = Player::get_friends(&player); // class::method(&instance)
    println!("Another way to get friends: {}", friends);

    // demo impl of enum
    let payment_mode = get_saved_payment_mode();
    payment_mode.pay(512);
}

// unit struct
struct Dummy;

// tuple struct
struct Color(u8, u8, u8);

// C lang struct
struct Player {
    name: String,
    iq: u8,
    friends: u8,
    score: u16
}

fn bump_player_score(mut player: Player, score: u16) {
    player.score += 120;
    println!("\nUpdated player stats:");
    println!("Name: {}", player.name);
    println!("IQ: {}", player.iq);
    println!("Friends: {}", player.friends);
    println!("Score: {}", player.score);
}

#[derive(Debug)]
enum Direction { N, E, S, W }

enum PlayerAction {
    Move {
        direction: Direction,
        speed: u8
    },
    Wait,
    Attack(Direction)
}

// add struct method with impl
impl Player {
    // can call without instance
    fn with_name(name: &str) -> Player {
        Player {
            name: name.to_string(),
            iq: 100,
            friends: 100,
            score: 1000
        }
    }

    // must call with an instance
    fn get_friends(&self) -> u8 {
        self.friends
    }

    fn set_friends(&mut self, count: u8) {
        self.friends = count;
    }
}

// add enum method with impl
enum PaymentMode {
    Debit,
    Credit,
    Paypal
}

fn pay_by_debit(amt: u64) {
    println!("Processing debit payment of {}", amt);
}

fn pay_by_credit(amt: u64) {
    println!("Processing credit payment of {}", amt);
}

fn paypal_redirect(amt: u64) {
    println!("Redirecting to paypal for amount: {}", amt);
}


impl PaymentMode {
    fn pay(&self, amount: u64) {
        match self {
            PaymentMode::Debit => pay_by_debit(amount),
            PaymentMode::Credit => pay_by_credit(amount),
            PaymentMode::Paypal => paypal_redirect(amount)
        }
    }
}

fn get_saved_payment_mode() -> PaymentMode {
    PaymentMode::Debit
}