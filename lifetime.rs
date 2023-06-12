fn main() {
    let string1 = String::from("Hello");
    let result;
    {
        let string2 = String::from("Rust");
        result = longest_string(&string1, &string2);
        println!("The longest string is: {}", result);
    }
    // The strings `string1` and `string2` have gone out of scope,
    // but `result` is still accessible since it has a longer lifetime.
}

fn longest_string<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}

/*
In this example, we have a main function that demonstrates the concept of lifetimes. We create two String variables, string1 and string2. Then, we define a block using curly braces to limit the scope of string2. Within that block, we call the longest_string function, passing references to string1 and string2.

The longest_string function has a generic lifetime parameter 'a, which allows us to specify that both input strings and the returned string reference should have the same lifetime. By doing so, we ensure that the returned reference will be valid as long as the input references are valid.

Inside the longest_string function, we compare the lengths of the two input strings and return a reference to the longest string. Since both input references have the same lifetime 'a, the returned reference will also have that lifetime.

Back in the main function, we assign the result of longest_string to the result variable, which is then printed out. Even though string1 and string2 have gone out of scope, result is still accessible because it has a longer lifetime.

By using lifetimes, Rust's borrow checker ensures that references are always valid, preventing dangling references and memory-related issues. It allows you to write safe and memory-efficient code while handling ownership and borrowing correctly.
*/
