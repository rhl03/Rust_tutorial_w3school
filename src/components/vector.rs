pub fn access_vector() {
    let fruits = vec!["apple", "banana", "orange"];
    println!("First fruit: {}", fruits[0]);
}
pub fn change_vector() {
    let mut fruits = vec!["apple", "banana", "orange"];
    fruits[0] = "grape";
    println!("New first fruit: {}", fruits[0]);
}
pub fn add_vector() {
    let mut fruits = vec!["apple", "banana"];
    fruits.push("cherry");
    println!("{:?}", fruits);
}
pub fn remove_vector() {
    let mut fruits = vec!["apple", "banana", "cherry"];
    fruits.pop();
    println!("{:?}", fruits);
}
pub fn insert_vector() {
    let mut fruits = vec!["banana", "orange"];
    fruits.insert(1, "apple");
    println!("{:?}", fruits);
}
pub fn lenght_vector() {
    let fruits = vec!["apple", "banana", "cherry"];
    println!("There are {} fruits.", fruits.len());
}
pub fn loop_vector() {
    let fruits = vec!["apple", "banana", "orange"];
    for fruit in &fruits {
        println!("I like {}.", fruit);
    }
}
