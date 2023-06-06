// Recursive function that calculates the factorial of a number
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1 // Base case: factorial of 0 is 1
    } else {
        n * factorial(n - 1) // Recursive case: factorial of n is n * factorial(n-1)
    }
}

fn main() {
    let number = 5;

    // Call the factorial function
    let result = factorial(number);

    println!("Factorial of {} is: {}", number, result);
}


/* Output
Factorial of 5 is: 120
*/
