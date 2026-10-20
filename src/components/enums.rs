enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn create_enum() {
    let my_direction = Direction::Left;

    match my_direction {
        Direction::Up => println!("Going up"),
        Direction::Down => println!("Going down"),
        Direction::Left => println!("Going left"),
        Direction::Right => println!("Going right"),
    }
}
enum LoginStatus {
    Success(String),
    Error(String),
}

pub fn data_enums() {
    let result1 = LoginStatus::Success(String::from("Welcome, John!"));
    let _result2 = LoginStatus::Error(String::from("Incorrect password"));

    match result1 {
        LoginStatus::Success(message) => println!("Success: {}", message),
        LoginStatus::Error(message) => println!("Error: {}", message),
    }
}
