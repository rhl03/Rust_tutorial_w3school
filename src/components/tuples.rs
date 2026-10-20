pub fn access_tuple() {
    let person = ("John", 30, true);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is active: {}", person.2);
}
pub fn unpack_typle() {
    let person = ("Jenny", 45, false);
    let (name, age, active) = person;

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Active: {}", active);
}
pub fn return_tuple() {
    let user = get_user();
    println!("User: {} ({} years old)", user.0, user.1);
}
fn get_user() -> (String, i32) {
    (String::from("Liam"), 25)
}
