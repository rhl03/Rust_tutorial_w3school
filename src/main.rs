mod components;

use components::{
    booleans, constants, data_types, if_else_conditions, match_tutorial, operators, variable,
};

// start of the program
fn main() {
    println!(" ");
    println!("w3school Rust tutorial: Variables");
    variable::variable();
    variable::variable_changes();
    println!(" ");
    println!("w3school Rust tutorial: data types");
    data_types::data_type();
    println!(" ");
    println!("w3school Rust tutorial: constants");
    constants::constant();
    println!(" ");
    println!("w3school Rust tutorial: operators");
    operators::arithmetic_operators();
    operators::assignment_operators();
    operators::comparison_operators();
    operators::logical_operators();
    println!(" ");
    println!("w3school Rust tutorial: booleans");
    booleans::boolean_comparison();
    booleans::boolean_if();
    booleans::boolean_specifyed();
    booleans::boolean_unspecifyed();
    println!(" ");
    println!("w3school Rust tutorial: booleans");
    if_else_conditions::if_condition();
    if_else_conditions::if_else_condition();
    if_else_conditions::else_if_condition();
    if_else_conditions::if_expression();
    println!(" ");
    println!("w3school Rust tutorial: match");
    match_tutorial::match_tutorial();
}
