use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let first_name = input.trim().to_string();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let last_name = input.trim().to_string();

    // Create username (lowercase, no space)
    let username = first_name + &last_name;
    let result = username.to_lowercase();

    // Create initials (uppercase first letters)
    let first_initial = username.chars().next().unwrap();
    let last_initial = last_name.chars().next().unwrap();
    // Create initials (uppercase first letters)
    println!("Username: {}", result);
    println!("Initials: {}{}", first_initial, last_initial);
}
