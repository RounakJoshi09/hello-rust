// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::Local;
fn main() {
    let current_time = Local::now().format("%d/%m/%Y");
    println!("{:?}", current_time.to_string());
}
