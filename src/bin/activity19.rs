// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();

    stock.insert("chair", 5);
    stock.insert("bed", 3);
    stock.insert("table", 2);
    stock.insert("couches", 0);

    let mut total_items = 0;

    for (stock_name, stock_size) in stock.iter() {
        let size = if stock_size == &0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", stock_size)
        };

        println!("name:{:?}, stock:{:?}", stock_name, size);

        total_items += stock_size;
    }

    println!("The total number of items is :{:?}", total_items);
}
