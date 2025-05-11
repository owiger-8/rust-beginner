use std::io;
fn main() {

    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Couldn't read ");

    if ispal(&s) {
        println!("palindrome");
    }else {
        println!("Not palindrome");
    }
}
fn ispal( t : &str) -> bool {
    let clean : String = t
    .chars()
    .filter(|c| c.is_alphanumeric())
    .map(|c| c.to_ascii_lowercase())
    .collect();

    let reversed : String = clean.chars().rev().collect();

    clean == reversed
}