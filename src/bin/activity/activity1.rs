// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

// Function to display the first name
fn display_first_name() -> String {
    "Rounak".to_string()
}

// Function to display the last name
fn display_last_name() -> String {
    "Joshi".to_string()
}

fn main() {
    // Use the println! macro to display the full name
    println!(
        "My name is {} {}",
        display_first_name(),
        display_last_name()
    );
}
