// Function that adds two numbers and returns the result
fn add_numbers(a: i32, b: i32) -> i32 {
    let sum = a + b;
    sum // Implicit return statement
}

fn main() {
    let num1 = 10;
    let num2 = 5;

    // Call the add_numbers function
    let result = add_numbers(num1, num2);

    println!("Result: {}", result);
}


/* Output
Result: 15
*/
