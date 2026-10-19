pub fn boolean_specifyed() {
    let is_programming_fun: bool = true;
    let is_fish_tasty: bool = false;

    println!("Is Programming Fun? {}", is_programming_fun);
    println!("Is Fish Tasty? {}", is_fish_tasty);
}
pub fn boolean_unspecifyed() {
    let is_programming_fun = true;
    let is_fish_tasty = false;

    println!("Is Programming Fun? {}", is_programming_fun);
    println!("Is Fish Tasty? {}", is_fish_tasty);
}
pub fn boolean_comparison() {
    let age = 20;
    let can_vote = age >= 18;
    println!("can vote? {}", can_vote);
}
pub fn boolean_if() {
    let is_logged_in = true;

    if is_logged_in {
        println!("Welcome back!");
    } else {
        println!("Please log in.");
    }
}
