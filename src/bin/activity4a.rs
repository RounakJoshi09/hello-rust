// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    let i_am_rouank = false;
    match i_am_rouank {
        true => println!("I am Rounak"),
        false => println!("I am not the body, I am not even the mind"),
    }
}
