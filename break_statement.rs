fn main() {
    let mut count = 0;

    loop {
        println!("Count: {}", count);
        count += 1;

        if count >= 5 {
            break; // Exit the loop
        }
    }
}


/* Output
Count: 0
Count: 1
Count: 2
Count: 3
Count: 4
*/
