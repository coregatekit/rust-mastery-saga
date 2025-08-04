// Macros in Rust are a powerful way to write code that generates other code.
// Types of macros in Rust

// Declarative macros (macro_rules!) basic macros like function that allow you to define patterns.
// println!(...);
// tokio::join!(...);

// Procedural macros are more powerful and flexible. They allow you to write code that operates on the abstract syntax tree (AST) of your code.
// #[tokio::main]
// #[derive(...)]

/// Import
use log_time::log_time; // Import the log_time macro from the log_time crate
/// 

// derive is used to automatically implement traits for a struct or enum.
// In this case, we derive the Debug trait to enable formatting for printing.
#[derive(Debug)]
struct Item {
    name: String,
    price: f64,
}

macro_rules! greet {
    // Macro frag spec: block | expr | expr_2021 | ident | path | tt | item | stmt | vis
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
    (crabby) => {
        println!("Hello from Crabby!");
    };
    () => {
        println!("Hello from the default macro!");
    };
}

#[log_time] // Apply the log_time macro to this function
fn main() {
    let item = Item {
        name: "Magic Wand".to_string(),
        price: 19.99
    };

    println!("Item: {:?}", item);

    greet!();
    greet!("World");
}
