fn main() {
    // Loop using 'for'
    let numbers = [1, 2, 3, 4, 5];

    for num in numbers.iter() {
        println!("Number: {}", num);
    }

    // Loop with a range
    for i in 0..5 {
        println!("Index: {}", i);
    }
}


/* Output
Number: 1
Number: 2
Number: 3
Number: 4
Number: 5
Index: 0
Index: 1
Index: 2
Index: 3
Index: 4
*/
