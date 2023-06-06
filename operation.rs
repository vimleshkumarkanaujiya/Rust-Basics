fn main() {
    // Arithmetic operations
    let num1 = 10;
    let num2 = 5;
    let sum = num1 + num2;
    let difference = num1 - num2;
    let product = num1 * num2;
    let quotient = num1 / num2;
    let remainder = num1 % num2;

    println!("Arithmetic Operations:");
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);

    // Comparison operations
    let a = 10;
    let b = 5;
    let c = 10;
    let greater_than = a > b;
    let less_than_or_equal = b <= c;
    let equal_to = a == c;
    let not_equal_to = a != b;

    println!("Comparison Operations:");
    println!("Greater Than: {}", greater_than);
    println!("Less Than or Equal: {}", less_than_or_equal);
    println!("Equal To: {}", equal_to);
    println!("Not Equal To: {}", not_equal_to);

    // Logical operations
    let p = true;
    let q = false;
    let logical_and = p && q;
    let logical_or = p || q;
    let logical_not = !p;

    println!("Logical Operations:");
    println!("Logical AND: {}", logical_and);
    println!("Logical OR: {}", logical_or);
    println!("Logical NOT: {}", logical_not);

    // Bitwise operations
    let x = 0b1010; // 10 in binary
    let y = 0b1100; // 12 in binary
    let bitwise_and = x & y;
    let bitwise_or = x | y;
    let bitwise_xor = x ^ y;
    let bitwise_shift_left = x << 2;
    let bitwise_shift_right = y >> 1;

    println!("Bitwise Operations:");
    println!("Bitwise AND: {:b}", bitwise_and);
    println!("Bitwise OR: {:b}", bitwise_or);
    println!("Bitwise XOR: {:b}", bitwise_xor);
    println!("Bitwise Shift Left: {:b}", bitwise_shift_left);
    println!("Bitwise Shift Right: {:b}", bitwise_shift_right);
}


/* Output
Arithmetic Operations:
Sum: 15
Difference: 5
Product: 50
Quotient: 2
Remainder: 0
Comparison Operations:
Greater Than: true
Less Than or Equal: true
Equal To: true
Not Equal To: true
Logical Operations:
Logical AND: false
Logical OR: true
Logical NOT: false
Bitwise Operations:
Bitwise AND: 8
Bitwise OR: 14
Bitwise XOR: 6
Bitwise Shift Left: 40
Bitwise Shift Right: 6
*/
