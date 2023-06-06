fn main() {
    // Integer types
    let int_num: i32 = 10;
    let uint_num: u32 = 20;

    // Floating-point types
    let float_num: f32 = 3.14;
    let double_num: f64 = 3.14159265359;

    // Boolean type
    let is_true: bool = true;

    // Character type
    let char_value: char = 'A';

    // String type
    let string_value: String = String::from("Hello, Rust!");

    // Array type
    let array_values: [i32; 3] = [1, 2, 3];

    // Tuple type
    let tuple_values: (i32, f32, bool) = (4, 5.5, false);

    // Display variable values
    println!("Integer: {}", int_num);
    println!("Unsigned Integer: {}", uint_num);
    println!("Float: {}", float_num);
    println!("Double: {}", double_num);
    println!("Boolean: {}", is_true);
    println!("Character: {}", char_value);
    println!("String: {}", string_value);
    println!("Array: {:?}", array_values);
    println!("Tuple: {:?}", tuple_values);
}


/* Output
Integer: 10
Unsigned Integer: 20
Float: 3.14
Double: 3.14159265359
Boolean: true
Character: A
String: Hello, Rust!
Array: [1, 2, 3]
Tuple: (4, 5.5, false)
*/
