pub fn ownership_example() {
    let a = String::from("Hello");
    let b = a;

    // println!("{}", a); Error: a no longer owns the value
    println!("{}", b); // Ok: b now owns the value 
}
pub fn ownership_clone() {
    let a = String::from("Hello");
    let b = a.clone();

    println!("a = {}", a);
    println!("b = {}", b);
}
