fn main() {
    // Loop using 'loop'
    let mut count = 0;
    loop {
        println!("Count: {}", count);
        count += 1;
        if count >= 5 {
            break; // Exit the loop
        }
    }

    // Loop using 'while'
    let mut i = 0;
    while i < 5 {
        println!("i: {}", i);
        i += 1;
    }
}


/* Output
Count: 0
Count: 1
Count: 2
Count: 3
Count: 4
i: 0
i: 1
i: 2
i: 3
i: 4
*/
