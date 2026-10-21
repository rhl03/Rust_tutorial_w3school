use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let celsius: i32 = input.trim().parse().unwrap();

    // Calculate fahrenheit
    let fahrenheit = celsius as f64 * 9.0 / 5.0 + 32.0;

    // Print the results
    println!("{} Celsius = {:.1} Fahrenheit", celsius, fahrenheit);
}
