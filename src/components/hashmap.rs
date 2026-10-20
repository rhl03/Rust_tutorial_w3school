use std::collections::HashMap;

pub fn create_hashmap() {
    let mut capital_cities = HashMap::new();

    // Add keys and values (Country, City)
    capital_cities.insert("England", "London");
    capital_cities.insert("Germany", "Berlin");
    capital_cities.insert("Norway", "Oslo");

    println!("{:?}", capital_cities);
}
pub fn access_hashmap() {
    let mut capital_cities = HashMap::new();

    capital_cities.insert("England", "London");
    capital_cities.insert("Germany", "Berlin");
    capital_cities.insert("Norway", "Oslo");

    if let Some(city) = capital_cities.get("England") {
        println!("The capital of England is {}.", city);
    } else {
        println!("England is not in the map.");
    }
}
pub fn update_hashmap() {
    let mut capital_cities = HashMap::new();

    capital_cities.insert("England", "London");
    capital_cities.insert("England", "Berlin");

    println!("{:?}", capital_cities);
}
pub fn remove_hashmap() {
    let mut capital_cities = HashMap::new();

    capital_cities.insert("England", "London");
    capital_cities.insert("Germany", "Berlin");
    capital_cities.insert("Norway", "Oslo");

    capital_cities.remove("England");
    println!("{:?}", capital_cities);
}
pub fn loop_hashmap() {
    let mut capital_cities = HashMap::new();

    capital_cities.insert("England", "London");
    capital_cities.insert("Germany", "Berlin");
    capital_cities.insert("Norway", "Oslo");

    for (country, city) in &capital_cities {
        println!("The capital of {} is {}.", country, city);
    }
}
