fn main() {
    println!("--- FizzBuzz: Rust Edition ---");

    // 1. The Range Loop
    // 1..=100 creates an inclusive range (1 to 100).
    // If we wrote 1..100, it would stop at 99.
    for i in 1..=100 {
        
        // 2. Pattern Matching on a Tuple
        // We create a temporary tuple: (remainder of 3, remainder of 5)
        match (i % 3, i % 5) {
            
            // Case A: Divisible by both 3 and 5 (0 remainder for both)
            (0, 0) => println!("FizzBuzz"),
            
            // Case B: Divisible by 3 only (0 remainder for 3, anything else for 5)
            // The '_' is a wildcard meaning "I don't care what this value is"
            (0, _) => println!("Fizz"),
            
            // Case C: Divisible by 5 only
            (_, 0) => println!("Buzz"),
            
            // Case D: Not divisible by either
            // We print the number itself.
            _ => println!("{}", i),
        }
    }
}