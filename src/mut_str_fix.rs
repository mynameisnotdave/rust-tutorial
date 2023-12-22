// Purpose of assignment: to understand how mutability and immutability defines
// how an item can be borrowed in Rust.

fn main() -> () {
    let mut s: String = String::from("hello world!");

    let word: &str = first_word(&s);
    println!("the first word is: {}", word);
    s.clear(); 
}

fn first_word(s: &str) -> &str {
    &s[..1]
}