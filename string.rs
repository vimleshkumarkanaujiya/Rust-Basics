fn main() {
    // Create an empty string
    let mut empty_string = String::new();

    // Create a string using a string literal
    let hello = "Hello, Rust!";
    println!("String literal: {}", hello);

    // Create a string from a string literal
    let greeting = String::from("Welcome to Rust!");
    println!("String from string literal: {}", greeting);

    // Concatenate strings
    let name = "Alice";
    let message = format!("Hello, {}!", name);
    println!("Formatted string: {}", message);

    // Modify a string
    empty_string.push_str("I am learning Rust!");
    println!("Modified string: {}", empty_string);

    // Access individual characters in a string
    let my_string = String::from("Rust");
    let third_char = my_string.chars().nth(2);
    match third_char {
        Some(c) => println!("Third character: {}", c),
        None => println!("String is empty!"),
    }
}


/* Output
String literal: Hello, Rust!
String from string literal: Welcome to Rust!
Formatted string: Hello, Alice!
Modified string: I am learning Rust!
Third character: s
*/
