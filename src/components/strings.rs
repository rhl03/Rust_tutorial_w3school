pub fn strings() {
    let greeting: &str = "Hello world";
    let text1 = "hello world".to_string();
    let text2 = String::from("hello world");

    println!("{}", text2);
    println!("{}", text1);
    println!("{}", greeting)
}

pub fn string_change() {
    let mut greeting = String::from("hello");
    greeting.push_str(" World");
    greeting.push('!');
    println!("{}", greeting);
}

pub fn concatenate_strings() {
    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = String::from("What a beautiful day!");
    let result = format!("{} {} {}", s1, s2, s3);
    println!("{}", result);
}

pub fn string_length() {
    let name = String::from("John");
    println!("Length: {}", name.len());
}
