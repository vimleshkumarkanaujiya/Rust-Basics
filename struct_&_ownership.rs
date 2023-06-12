struct Person {
    name: String,
    age: u32,
    is_student: bool,
}

impl Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}.", self.name);
    }
}

fn main() {
    let mut person1 = Person {
        name: String::from("Alice"),
        age: 25,
        is_student: true,
    };

    take_ownership(&mut person1);

    person1.say_hello();
}

fn take_ownership(person: &mut Person) {
    person.name = String::from("Bob");
    person.age = 30;
    person.is_student = false;
}

/*
In this example, we define a Person struct with the name, age, and is_student fields, along with an associated say_hello method. The main function creates a person1 object of type Person, initializes its fields, and calls the take_ownership function, passing a mutable reference to person1.

The take_ownership function takes a mutable reference to a Person object and modifies its fields. In this case, it changes the person's name, age, and student status. After the function call, the person1 object still retains ownership, but its fields have been modified.

Finally, the say_hello method is called on the person1 object, which prints a greeting message with the person's updated name.

This example demonstrates the combination of struct definition, method implementation, ownership passing via references, and modifying the struct's fields. It showcases the use of structs to represent real-world entities and the ability to define behavior associated with the struct using methods.
*/
