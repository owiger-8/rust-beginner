use std::env; 
use std::fs; 
use std::process; 

fn main() {
    
    let args: Vec<String> = env::args().collect();


    if args.len() < 2 {
        eprintln!("Usage: cargo run <filename>");
        process::exit(1);
    }

    let filename = &args[1];

    println!("Reading file: {}", filename);

    
    let content = match fs::read_to_string(filename) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(1);
        }
    };

  
    let line_count = content.lines().count();

    let word_count = content.split_whitespace().count();

    let byte_count = content.len(); 

    println!("--------------------------------");
    println!("Lines:\t{}", line_count);
    println!("Words:\t{}", word_count);
    println!("Bytes:\t{}", byte_count);
    println!("--------------------------------");
}