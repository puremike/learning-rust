use std::io;

pub fn get_color(colors: &[&str]) -> String {
    println!("Enter a number between 1 to {}", colors.len());
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let index: usize = match input.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= colors.len() => num - 1, // adjust for 0-index
        _ => {
            println!("Invalid input. Enter a number in range");
            return "".to_string();
        }
    };
    colors[index].into()
}

pub fn use_color() {
    let color: Vec<&str> = vec!["Red", "Green", "Blue", "Yellow", "Black"];
    println!("Your favorite color is {}", get_color(&color));
}
