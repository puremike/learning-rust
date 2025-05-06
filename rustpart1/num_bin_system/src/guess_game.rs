use rand::Rng;
use std::io;

pub fn guess_number() {
    println!("Guessing Game");
    println!("Enter a number between 1 to 100: ");

    let mut number: String = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    // let numner: i32 = number.trim().parse().unwrap();

    let number: i32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number in range");
            return;
        }
    };

    assert!(number >= 1 && number <= 100, "Number out of range");
    println!("You guessed: {}", number);
}

pub fn guess_number_random() {
    println!("Guessing Game from Random Number");

    let number: i32 = rand::rng().random_range(1..=100);
    println!("You guessed: {}", number);
}
