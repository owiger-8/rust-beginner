use std::io; // for the input and output ig

fn main() {
    println!("--- Temperature Converter ---");
    println!("Type '1' for Fahrenheit to Celsius");
    println!("Type '2' for Celsius to Fahrenheit");

    println!("Please enter your choice:");
    
    // this variable is mutable which means it can update its value 
    let mut choice = String::new();

    // using the imported package reading the input
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    println!("Enter the temperature value:");
    
    // The temperature is a string value 
    let mut temp_str = String::new();
    io::stdin()
        .read_line(&mut temp_str)
        .expect("Failed to read line");

    // converting string to float
    let temp: f64 = match temp_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("That is not a valid number!");
            return; // not a float
        }
    };

    // checking the choices using if else 
    if choice.trim() == "1" {
        // Formula: (F - 32) * 5/9
        let result = (temp - 32.0) * 5.0 / 9.0;
        // {:.2} formats the number to 2 decimal places
        println!("{:.2}°F is {:.2}°C", temp, result);
    } else if choice.trim() == "2" {
        // Formula: (C * 9/5) + 32
        let result = (temp * 9.0 / 5.0) + 32.0;
        println!("{:.2}°C is {:.2}°F", temp, result);
    } else {
        println!("Invalid choice selected.");
    }
}