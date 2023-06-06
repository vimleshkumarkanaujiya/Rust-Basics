// Define a struct called `Person`
struct Person {
    name: String,
    age: u32,
    is_active: bool,
}

fn main() {
    // Create an instance of the `Person` struct
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
        is_active: true,
    };

    // Access fields of the struct
    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
    println!("Is Active? {}", person1.is_active);

    // Create a mutable instance of the struct
    let mut person2 = Person {
        name: String::from("Bob"),
        age: 25,
        is_active: false,
    };

    // Modify fields of the struct
    person2.age = 27;
    person2.is_active = true;

    // Print the modified values
    println!("Name: {}", person2.name);
    println!("Age: {}", person2.age);
    println!("Is Active? {}", person2.is_active);
}


/* Output
Name: Alice
Age: 30
Is Active? true
Name: Bob
Age: 27
Is Active? true
*/
