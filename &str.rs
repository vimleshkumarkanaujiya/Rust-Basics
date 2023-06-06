fn main() {
    // Create a string slice from a string literal
    let greeting: &str = "Hello, Rust!";
    println!("String slice: {}", greeting);

    // Create a string slice from an existing String
    let message: String = String::from("Welcome to Rust!");
    let message_slice: &str = &message;
    println!("String slice from String: {}", message_slice);

    // Concatenate string slices
    let name: &str = "Alice";
    let full_message: &str = &format!("Hello, {}!", name);
    println!("Formatted string slice: {}", full_message);

    // Access individual characters in a string slice
    let my_string: &str = "Rust";
    let third_char: Option<char> = my_string.chars().nth(2);
    match third_char {
        Some(c) => println!("Third character: {}", c),
        None => println!("String is empty!"),
    }
}


/* Output
String slice: Hello, Rust!
String slice from String: Welcome to Rust!
Formatted string slice: Hello, Alice!
Third character: s
*/
