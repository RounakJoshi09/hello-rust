// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

trait Body {
    fn get_body_material(&self) -> String;
}
trait Color {
    fn get_body_color(&self) -> String;
}

struct VehicleBody {
    material: String,
}

struct VehicleColor {
    color: String,
}

impl Body for VehicleBody {
    fn get_body_material(&self) -> String {
        return self.material.to_owned();
    }
}

impl Color for VehicleColor {
    fn get_body_color(&self) -> String {
        return self.color.to_owned();
    }
}

struct Vehicle<T, U>
where
    T: Body,
    U: Color,
{
    body: T,
    color: U,
}

impl<T: Body, U: Color> Vehicle<T, U> {
    fn new(body: T, color: U) -> Self {
        return Self {
            color: color,
            body: body,
        };
    }

    fn print_vehicle_color(&self) {
        println!("{:?}", self.color.get_body_color());
    }
    fn print_vehicle_material(&self) {
        println!("{:?}", self.body.get_body_material());
    }

    fn print_vehicle_details(&self) {
        self.print_vehicle_color();
        self.print_vehicle_material();
    }
}

fn main() {
    let steel_body = VehicleBody {
        material: "Steel".to_owned(),
    };
    let iron_body = VehicleBody {
        material: "Iron".to_owned(),
    };

    let red_color = VehicleColor {
        color: "Red".to_owned(),
    };

    let green_color = VehicleColor {
        color: "Green".to_owned(),
    };

    let maruti_suzuki_800 = Vehicle::new(steel_body, red_color);

    let maruti_alto = Vehicle::new(iron_body, green_color);

    maruti_suzuki_800.print_vehicle_details();

    maruti_alto.print_vehicle_details();
}
