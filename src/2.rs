use std::io;

fn main() {
    let mut guess = String::new();
    println!("Guess a number between 1 and 100: ");
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    if guess < 1 || guess > 100 {
        println!("Your guess is out of bounds. It must be between 1 and 100.");
        return;
    }

    // Check if the number is even or odd
    let result = if guess % 2 == 0 { "even" } else { "odd" };
    println!("Your guess {} is {}", guess, result);
}
