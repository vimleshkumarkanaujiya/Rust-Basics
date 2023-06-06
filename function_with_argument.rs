// Function that calculates the area of a rectangle given its width and height
fn calculate_area(width: f64, height: f64) -> f64 {
    let area = width * height;
    area // Implicit return statement
}

fn main() {
    let width = 5.0;
    let height = 3.5;

    // Call the calculate_area function with width and height as arguments
    let result = calculate_area(width, height);

    println!("Area: {}", result);
}


/* Output
Area: 17.5
*/
