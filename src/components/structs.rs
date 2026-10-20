pub fn create_structs() {
    struct Person {
        name: String,
        age: u32,
        can_vote: bool,
    }

    let user = Person {
        name: String::from("John"),
        age: 35,
        can_vote: true,
    };

    println!("Name: {}", user.name);
    println!("Age: {}", user.age);
    println!("Can vote? {}", user.can_vote);
}
pub fn change_structs() {
    struct Person {
        name: String,
        age: u32,
    }

    let mut user = Person {
        name: String::from("John"),
        age: 35,
    };

    user.age = 36; // Change value of age
    println!("Name: {}", user.name);
    println!("Updated age: {}", user.age);
}
