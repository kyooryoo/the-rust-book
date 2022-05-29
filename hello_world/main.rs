fn main() {
    println!("Hello, world!");

    let target = "world";
    let mut greeting = "Hello";
    println!("{}, {}!", greeting, target);
    greeting = "How are you doing!";
    // cannot assign twice to immutable variable
    target = "mate";
    println!("{}, {}!", greeting, target);
}