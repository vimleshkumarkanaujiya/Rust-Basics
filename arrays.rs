fn main() {
    // Create an array with explicit type annotation
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Access elements of the array
    println!("First element: {}", numbers[0]);
    println!("Second element: {}", numbers[1]);

    // Iterate over the array
    for num in numbers.iter() {
        println!("Element: {}", num);
    }

    // Create an array with shorthand syntax
    let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    // Access elements of the array
    println!("First day of the week: {}", weekdays[0]);
    println!("Second day of the week: {}", weekdays[1]);

    // Iterate over the array
    for day in weekdays.iter() {
        println!("Day: {}", day);
    }
}


/* Output
First element: 1
Second element: 2
Element: 1
Element: 2
Element: 3
Element: 4
Element: 5
First day of the week: Monday
Second day of the week: Tuesday
Day: Monday
Day: Tuesday
Day: Wednesday
Day: Thursday
Day: Friday
*/
