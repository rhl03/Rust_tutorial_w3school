pub fn create_array() {
    let numbers = [1, 2, 3, 4, 5];
    println!("The first number is: {}", numbers[0]);
}

pub fn access_array() {
    let numbers = [1, 2, 3, 4, 5];
    println!("The first number is: {}", numbers[0]);
}

pub fn change_array() {
    let mut numbers = [1, 2, 3, 4, 5];
    numbers[0] = 10;
    println!("The new first number is: {}", numbers[0]);
}

pub fn length_array() {
    let numbers = [1, 2, 3, 4, 5];
    println!("This array has {} elements.", numbers.len());
}

pub fn loop_array() {
    let fruits = ["apple", "banana", "orange"];
    for fruit in fruits {
        println!("I like {}.", fruit);
    }
}

pub fn print_array() {
    let numbers = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
}

pub fn dynamic_array() {
    let mut cars = vec!["Volvo", "BMW", "Ford"];
    cars.push("Mazda");
    println!("{:?}", cars);
}
