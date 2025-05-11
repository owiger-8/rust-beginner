use std::io;
fn main() {

    let mut s = String::new();
    io::stdin()
        .readling(&mut s);
        .except("Couldn't read ");

    if ispal(&input) {
        println("palindrome");
    }else {
        println("Not palindrome");
    }
}
fn ispal( t : &str) -> bool {
    let clean : String = t
    .chars()
    .filter(|c| c.is_alphanumeric())
    .map(|c| c.to_ascii_lowercase())
    .collect();

    let reversed : String = cleaned.chars().rev().collect();

    cleaned == reversed
}