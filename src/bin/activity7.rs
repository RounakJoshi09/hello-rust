// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
}

fn print_color_name(color: Color) {
    match color {
        Color::Blue => println!("This is Blue"),
        Color::Green => println!("This is Green"),
        Color::Red => println!("This is Red"),
        Color::Yellow => println!("This is Yellow"),
        _ => println!("I dont know this color"),
    }
}

fn main() {
    let color: Color = Color::Blue;
    print_color_name(color);
}
