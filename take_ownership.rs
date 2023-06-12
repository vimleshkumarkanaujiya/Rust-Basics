fn main() {
    // Create a string and assign it to the variable "original".
    let original = String::from("Hello, Rust!");

    // Pass ownership of the string to the "take_ownership" function.
    take_ownership(original);

    // This line would result in a compilation error since the ownership has been transferred.
    // println!("The original string: {}", original);

    // Create a new string and assign it to the variable "borrowed".
    let borrowed = String::from("Hello, borrowing!");

    // Pass a reference of the string to the "borrow" function.
    borrow(&borrowed);

    // The "borrowed" string is still accessible since we only borrowed it.
    println!("The borrowed string: {}", borrowed);
}

fn take_ownership(some_string: String) {
    // The function takes ownership of the string parameter.
    println!("I took ownership: {}", some_string);
    // When the function goes out of scope, the string is dropped and its memory is freed.
}

fn borrow(some_string: &String) {
    // The function borrows a reference to the string parameter.
    println!("I borrowed: {}", some_string);
    // We can only read the borrowed string; modifying it would result in a compilation error.
}
