use std::io ; 
fn main() {
    
    
    let mut s = String::new()  ;
    io::stdin()
        .read_line(&mut s)
        .expect("no input");

    let count = vowels(&s);
    println!( "Number of vowels {}" . count );
} 
fn count ( sen : String) -> usize {
    let mut count = 0 ;

    for c in sen.chars(){
        match c.to_ascii_lowercase(){
            'a' | 'e' | 'i' | 'o' | 'u'  => {
                count += 1;
            }

        }

    }
    count
}
