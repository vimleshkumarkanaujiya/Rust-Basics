fn main() {
    for num in 1..=10 {
        if num % 2 == 0 {
            continue; // Skip even numbers
        }

        println!("Number: {}", num);
    }
}


/* Output
Number: 1
Number: 3
Number: 5
Number: 7
Number: 9
*/
