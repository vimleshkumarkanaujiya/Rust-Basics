fn main() {
    let mut my_string = String::from("Hello, Rust!");

    println!("Length: {}", my_string.len());
    println!("Is empty? {}", my_string.is_empty());

    my_string.push_str(" Welcome!");
    println!("Appended: {}", my_string);

    my_string.pop();
    println!("After pop: {}", my_string);

    my_string = my_string.replace("Rust", "World");
    println!("Replaced: {}", my_string);

    println!("Lowercase: {}", my_string.to_lowercase());
    println!("Uppercase: {}", my_string.to_uppercase());

    for word in my_string.split_whitespace() {
        println!("Word: {}", word);
    }

    println!("Contains 'World'? {}", my_string.contains("World"));
    println!("Starts with 'Hello'? {}", my_string.starts_with("Hello"));
    println!("Ends with 'Welcome'? {}", my_string.ends_with("Welcome"));

    my_string = String::from("   Trimmed    ");
    println!("Before trim: '{}'", my_string);
    println!("After trim: '{}'", my_string.trim());

    for c in my_string.chars() {
        println!("Character: {}", c);
    }

    for b in my_string.bytes() {
        println!("Byte: {}", b);
    }
}


/* Output
Length: 13
Is empty? false
Appended: Hello, Rust! Welcome!
After pop: Hello, Rust! Welcome
Replaced: Hello, World! Welcome
Lowercase: hello, world! welcome
Uppercase: HELLO, WORLD! WELCOME
Word: Hello,
Word: World!
Word: Welcome
Contains 'World'? true
Starts with 'Hello'? true
Ends with 'Welcome'? true
Before trim: '   Trimmed    '
After trim: 'Trimmed'
Character: ' '
Character: ' '
Character: ' '
Character: 'T'
Character: 'r'
Character: 'i'
Character: 'm'
Character: 'm'
Character: 'e'
Character: 'd'
Byte: 32
Byte: 32
Byte: 32
Byte: 84
Byte: 114
Byte: 105
Byte: 109
Byte: 109
Byte: 101
Byte: 100
*/
