use std::env; // Environment handling (for CLI arguments)
use std::fs;  // Filesystem handling
use std::process; // For exiting the program gracefully

fn main() {
    // 1. Collect Command Line Arguments
    // env::args() returns an iterator of the arguments
    // .collect() turns that iterator into a Vector (Vec<String>)
    let args: Vec<String> = env::args().collect();

    // Check if the user provided a filename
    // args[0] is the name of the program itself
    // args[1] is the first argument provided by the user
    if args.len() < 2 {
        eprintln!("Usage: cargo run <filename>");
        process::exit(1); // Exit with an error code
    }

    let filename = &args[1];

    println!("Reading file: {}", filename);

    // 2. Read the file content
    // fs::read_to_string opens the file and reads it entirely into memory.
    // If it fails (e.g., file doesn't exist), expect() crashes with a message.
    let content = match fs::read_to_string(filename) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    // 3. Calculate Statistics
    // .lines() creates an iterator over lines
    let line_count = content.lines().count();
    
    // .split_whitespace() splits by spaces, tabs, and newlines
    let word_count = content.split_whitespace().count();
    
    // .len() returns the size in bytes
    let byte_count = content.len(); 

    // 4. Display Results
    println!("--------------------------------");
    println!("Lines:\t{}", line_count);
    println!("Words:\t{}", word_count);
    println!("Bytes:\t{}", byte_count);
    println!("--------------------------------");
}