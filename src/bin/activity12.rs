// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Blue,
    Red,
    Purple,
    Yellow,
}

struct ShippingBox {
    dimension: i32,
    weight: i32,
    color: Color,
}

impl Color {
    fn print_color(&self) {
        match self {
            Color::Blue => println!("Blue"),
            Color::Purple => println!("Purple"),
            Color::Yellow => println!("Yellow"),
            Color::Red => println!("Red"),
        }
    }
}

impl ShippingBox {
    fn create_shipping_box() -> Self {
        return Self {
            color: Color::Purple,
            dimension: 30,
            weight: 15,
        };
    }

    fn print_shipping_details(&self) {
        println!("{0} {1}", self.dimension, self.weight);
        self.color.print_color();
    }
}

fn main() {
    let flipkart = ShippingBox {
        color: Color::Purple,
        dimension: 90,
        weight: 30,
    };
    let amazon = ShippingBox::create_shipping_box();

    amazon.print_shipping_details();

    flipkart.print_shipping_details();
}
