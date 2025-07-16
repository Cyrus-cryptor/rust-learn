use std::string;

fn main() {
    let dangling = dangle();
}

fn takes_ownership(some_string: String) {
    println!("{some_string}")
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}")
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    return (s, length);
}
fn calculate_length_with_reference(s: &String) -> usize {
    s.len()
}

fn dangle() -> &String {
    let s = String::from("Dangling");
    return &s;
}
