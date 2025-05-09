use std::io;

fn main() {
    println!("--- Recursive Factorial Calculator ---");
    println!("Enter a positive integer (0-34):");

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parsing into u128 (Unsigned 128-bit integer)
    let n: u128 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid positive number.");
            return;
        }
    };

    // Factorials grow very fast! u128 can only hold up to 34!
    if n > 34 {
        println!("Sorry, that number is too big for this computer to handle!");
        return;
    }

    let result = factorial(n);
    println!("The factorial of {} is: {}", n, result);
}

// --- The Recursive Function ---
// This function calls itself until it hits the "base case"
fn factorial(n: u128) -> u128 {
    // Base Case: If n is 0, stop recursing and return 1.
    if n == 0 {
        1
    } else {
        // Recursive Step: n * factorial of (n-1)
        n * factorial(n - 1)
    }
}