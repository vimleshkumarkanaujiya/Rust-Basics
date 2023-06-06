fn main() {
    // Create a tuple with explicit type annotation
    let person: (String, i32, bool) = ("Alice".to_string(), 30, true);

    // Access elements of the tuple
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is Active? {}", person.2);

    // Destructure the tuple into individual variables
    let (name, age, is_active) = person;

    // Print the values using individual variables
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is Active? {}", is_active);

    // Create a tuple with shorthand syntax
    let point = (3.5, 2.0);

    // Access elements of the tuple
    println!("X-coordinate: {}", point.0);
    println!("Y-coordinate: {}", point.1);
}


/* Output
Name: Alice
Age: 30
Is Active? true
Name: Alice
Age: 30
Is Active? true
X-coordinate: 3.5
Y-coordinate: 2
*/
