use std::io;

fn main() {
    println!("--- Rust Calculator ---");

    // 1. Get the first number
    println!("Enter first number:");
    let num1_str = read_input();
    
    // Parse string to f64 (double precision float)
    let num1: f64 = match num1_str.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number!");
            return;
        }
    };

    // 2. Get the operation
    println!("Enter operation (+, -, *, /):");
    let operator = read_input();

    // 3. Get the second number
    println!("Enter second number:");
    let num2_str = read_input();
    
    let num2: f64 = match num2_str.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number!");
            return;
        }
    };

    // 4. Perform the logic
    // We match against the string slice (&str)
    match operator.as_str() {
        "+" => println!("Result: {}", num1 + num2),
        "-" => println!("Result: {}", num1 - num2),
        "*" => println!("Result: {}", num1 * num2),
        "/" => {
            if num2 == 0.0 {
                println!("Error: Cannot divide by zero.");
            } else {
                println!("Result: {}", num1 / num2);
            }
        }
        _ => println!("Invalid operator! Please use +, -, *, or /"),
    }
}

// --- Helper Function ---
// This function reads a line from the user, removes whitespace, and returns a String.
fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    // trim() removes the newline character ("\n") from hitting Enter
    // to_string() converts the string slice back into an owned String
    input.trim().to_string()
}