use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    // Check if even or odd and print

    if n % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    };
}
