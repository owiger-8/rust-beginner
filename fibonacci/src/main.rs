
struct Fibonacci {
    curr: u64,
    next: u64,
}


impl Iterator for Fibonacci {
    
    type Item = u64;

    
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        let current_value = self.curr;

  
        self.curr = self.next;
        self.next = new_next;

        Some(current_value)
    }
}


fn fibonacci_iter() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    println!("--- Approach 1: The Loop (Imperative) ---");
    let n = 10;
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        print!("{} ", a);
        let temp = a;
        a = b;
        b = temp + b;
    }
    println!("\n"); 

    println!("--- Approach 2: The Iterator (Functional) ---");

    fibonacci_iter().take(10).for_each(|num| print!("{} ", num));
    
    println!("\n");
}