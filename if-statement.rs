fn main() {
    let num = 10;
    // Check if the number is greater than 5
    if num > 5 {
        println!("The number is greater than 5");
    } else {
        println!("The number is not greater than 5");
    }
    // Check if the number is positive, negative, or zero
    if num > 0 {
        println!("The number is positive");
    } else if num < 0 {
        println!("The number is negative");
    } else {
        println!("The number is zero");
    }
    // Check if the number is even or odd
    if num % 2 == 0 {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }
}


/* Output
The number is greater than 5
The number is positive
The number is even
*/                     
                     
