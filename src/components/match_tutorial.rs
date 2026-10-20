pub fn match_tutorial() {
    let day = 4;
    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }
}

pub fn multiple_matches() {
    let day = 6;

    match day {
        1..=5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }
}

pub fn match_with_return() {
    let day = 4;

    let result = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day.",
    };

    println!("{}", result);
}
