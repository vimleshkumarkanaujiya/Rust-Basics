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


/*
In this example, we start by creating a String called original and pass its ownership to the take_ownership function. Inside the function, we can manipulate the string as needed. However, once the function call is complete, the ownership of original is transferred, and trying to access it again would result in a compilation error.

Next, we create a new String called borrowed and pass a reference to it (&borrowed) to the borrow function. This allows the borrow function to access and read the string without taking ownership. The borrowed string remains accessible even after the function call, demonstrating how borrowing works in Rust.

Keep in mind that this is a basic example to illustrate the concept of ownership and borrowing in Rust. In real-world scenarios, you would often encounter more complex ownership relationships and patterns.
*/
