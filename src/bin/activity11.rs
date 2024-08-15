// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    id: i32,
    quantity: i32,
}

fn print_quantity(grocery_item: &GroceryItem) {
    println!("{}", grocery_item.quantity)
}

fn print_id(grocery_item: &GroceryItem) {
    println!("{}", grocery_item.id)
}

fn main() {
    let grocery_item = GroceryItem {
        id: 0,
        quantity: 15,
    };

    print_quantity(&grocery_item);
    print_id(&grocery_item);
}
