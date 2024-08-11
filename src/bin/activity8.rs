// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum DrinkFlavour {
    VANILLA,
    CHOCLATE,
    PINEAPPLE,
    APPLEVINEGAR,
}

struct DrinkInfo {
    flavour: DrinkFlavour,
    ounce: f64,
}

fn print_drink_info(drink_info: DrinkInfo) {
    match drink_info.flavour {
        DrinkFlavour::APPLEVINEGAR => println!("Apple  Vinegar"),
        DrinkFlavour::CHOCLATE => println!("Choclate"),
        DrinkFlavour::PINEAPPLE => println!("Pineapple"),
        DrinkFlavour::VANILLA => println!("Vanilla"),
    }

    println!("The quantity is {:?}", drink_info.ounce);
}

fn main() {
    let cock_tail: DrinkInfo = DrinkInfo {
        ounce: 20.0,
        flavour: DrinkFlavour::CHOCLATE,
    };

    print_drink_info(cock_tail);
}
