// Define an enum called `Color`
enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
}

fn main() {
    // Create instances of the `Color` enum
    let color1 = Color::Red;
    let color2 = Color::Rgb(0, 255, 0);

    // Match on the enum variant
    match color1 {
        Color::Red => println!("Color is Red!"),
        Color::Green => println!("Color is Green!"),
        Color::Blue => println!("Color is Blue!"),
        Color::Rgb(r, g, b) => println!("Color is RGB({}, {}, {})", r, g, b),
    }

    match color2 {
        Color::Red => println!("Color is Red!"),
        Color::Green => println!("Color is Green!"),
        Color::Blue => println!("Color is Blue!"),
        Color::Rgb(r, g, b) => println!("Color is RGB({}, {}, {})", r, g, b),
    }
}


/* Output
Color is Red!
Color is RGB(0, 255, 0)
*/
