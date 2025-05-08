use std::io;

fn main() {
    println!("--- Rust Calculator ---");

    println!("Enter first number:");
    let num1_str = read_input();
    
    let num1: f64 = match num1_str.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number!");
            return;
        }
    };

  
    println!("Enter operation (+, -, *, /):");
    let operator = read_input();

    println!("Enter second number:");
    let num2_str = read_input();
    
    let num2: f64 = match num2_str.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number!");
            return;
        }
    };

  
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


fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}