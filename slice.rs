fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // Create a slice that includes the entire array
    let whole_slice: &[i32] = &numbers;
    println!("Whole Slice: {:?}", whole_slice);

    // Create a slice that includes a subset of the array
    let partial_slice: &[i32] = &numbers[1..4];
    println!("Partial Slice: {:?}", partial_slice);

    // Iterate over the elements in the slice
    for num in whole_slice {
        println!("Element: {}", num);
    }

    // Update the values in the original array
    // This will also reflect in the slice
    numbers[2] = 10;

    // Print the modified slice
    println!("Modified Slice: {:?}", whole_slice);
}


/* Output
Whole Slice: [1, 2, 3, 4, 5]
Partial Slice: [2, 3, 4]
Element: 1
Element: 2
Element: 3
Element: 4
Element: 5
Modified Slice: [1, 2, 10, 4, 5]
*/
