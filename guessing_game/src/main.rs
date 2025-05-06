use std::io;
use std::cmp::Ordering; // Used to compare numbers (Less, Greater, Equal)
use rand::Rng;          // Trait needed to generate random numbers

fn main() {
    println!("--- Guess the Number Game ---");

    // 1. Generate a random number between 1 and 100
    // thread_rng() gives us a random number generator local to the current thread
    // gen_range(1..=100) is inclusive (1 to 100)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {}", secret_number); // Uncomment for testing

    println!("I have picked a number between 1 and 100.");

    // 2. Start an infinite loop
    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 3. Shadowing and Error Handling
        // We 'shadow' the previous 'guess' string variable with a new 'guess' u32 variable.
        // match allows us to handle non-number inputs without crashing.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue; // Skip the rest of the loop and start over
            }
        };

        println!("You guessed: {}", guess);

        // 4. Compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Exit the loop
            }
        }
    }
}