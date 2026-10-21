use std::io;

fn main() {
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let age: i32 = input.trim().parse().unwrap();

    // Check and print
    if age >= 18 {
        println!("{} can vote", name);
    } else {
        println!("{} cannot vote", name);
    }
}
