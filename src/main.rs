mod components;

use components::{
    arrays, booleans, borrorwing, constants, data_types, function, if_else_conditions, loops,
    match_tutorial, operators, ownership, strings, variable,
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
    match_tutorial::multiple_matches();
    match_tutorial::match_with_return();
    println!(" ");
    println!("w3school Rust tutorial: loops");
    loops::loops();
    loops::loop_with_value();
    loops::while_loop();
    loops::while_loop_break();
    loops::skip_value();
    loops::for_loop();
    loops::inclusive_loop();
    loops::break_continue_loop();
    println!(" ");
    println!("w3school Rust tutorial: functions");
    function::functions_with_return();
    println!(" ");
    println!("w3school Rust tutorial: scope");
    strings::strings();
    strings::string_change();
    strings::concatenate_strings();
    strings::string_length();
    println!(" ");
    println!("w3school Rust tutorial: ownership");
    ownership::ownership_example();
    ownership::ownership_clone();
    println!("w3school Rust tutorial: borrorwing");
    borrorwing::borrowing();
    borrorwing::mutable_borrowing();
    println!(" ");
    println!("w3school Rust tutorial: arrays");
    arrays::create_array();
    arrays::access_array();
    arrays::change_array();
    arrays::length_array();
    arrays::loop_array();
    arrays::print_array();
    arrays::dynamic_array();
}
