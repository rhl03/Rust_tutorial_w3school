pub fn if_condition() {
    let x = 7;
    let y = 5;

    if x > y {
        println!("x is greater than y.");
    }
    if 7 > 6 {
        println!("7 is greater then 6.");
    }
}
pub fn if_else_condition() {
    let age = 16;

    if age >= 18 {
        println!("You can vote.");
    } else {
        println!("You are too young to vote.");
    }
}

pub fn else_if_condition() {
    let score = 85;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }
}
pub fn if_expression() {
    let time = 20;
    let greeting = if time < 18 {
        "Good day."
    } else {
        "Good evening."
    };
    println!("{}", greeting);
}
