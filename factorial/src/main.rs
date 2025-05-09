use std::io;

fn main() {
    println!("--- Recursive Factorial Calculator ---");
    println!("Enter a positive integer (0-34):");

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u128 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid positive number.");
            return;
        }
    };

    
    if n > 34 {
        println!("Too big for recursion");
        return;
    }

    let result = factorial(n);
    println!("The factorial of {} is: {}", n, result);
}

fn factorial(n: u128) -> u128 {
   
    if n == 0 {
        1
    } else {
    
        n * factorial(n - 1)
    }
}