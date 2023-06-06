// Function that checks if a number is positive and returns a boolean
fn is_positive(number: i32) -> bool {
    if number > 0 {
        return true;  // Return true if the number is positive
    } else {
        return false; // Return false if the number is non-positive
    }
}

fn main() {
    let num1 = 10;
    let num2 = -5;

    // Call the is_positive function and store the result in variables
    let result1 = is_positive(num1);
    let result2 = is_positive(num2);

    println!("Is {} positive? {}", num1, result1);
    println!("Is {} positive? {}", num2, result2);
}


/* Output
Is 10 positive? true
Is -5 positive? false
*/
