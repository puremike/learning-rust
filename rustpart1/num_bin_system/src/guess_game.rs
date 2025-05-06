use rand::Rng;
use std::{cmp::Ordering, io};

pub fn guess_number() {
    println!("Guessing Game");

    // loop {} everything below can be wrapped under the loop block to make the guessing game continuous
    // Ordering::Equal => {
    //     println!("You win!");
    // break;
    // }

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

    let random_number: i32 = rand::rng().random_range(1..=100);
    println!("Random number: {}\n", random_number);

    // Compare the user input to the random_number
    match number.cmp(&random_number) {
        Ordering::Less => print!("Too small!"),
        Ordering::Greater => print!("Too big!"),
        Ordering::Equal => print!("You win!"),
    }
}
