pub fn borrowing() {
    let a = String::from("Hello");
    let b = &a;

    println!("a = {}", a);
    println!("b = {}", b);
}

pub fn mutable_borrowing() {
    let mut name = String::from("John");
    let name_ref = &mut name;
    name_ref.push_str(" Doe");

    println!("{}", name_ref);
}
