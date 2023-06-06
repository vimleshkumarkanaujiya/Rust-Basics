fn main() {
    let num = 10;

    if num > 5 {
        println!("The number is greater than 5");

        if num > 7 {
            println!("The number is also greater than 7");
        } else {
            println!("The number is not greater than 7");
        }
    } else {
        println!("The number is less than or equal to 5");
    }
}


/* Output
The number is greater than 5
The number is also greater than 7
*/
