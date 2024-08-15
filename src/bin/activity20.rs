// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

enum Ops {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn get_user_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    return Ok(buffer.trim().to_owned());
}

impl Ops {
    fn take_action(&self) {
        match self {
            Ops::Hibernate => println!("Starting Hibernate"),
            Ops::Off => println!("Audios Amigo Powering Off"),
            Ops::Reboot => println!("Rebooting"),
            Ops::Shutdown => println!("Shutting Down"),
            Ops::Sleep => println!("Going to Sleep , Good Night"),
        }
    }
}

fn main() {
    // let user_input = get_user_input();

    while let result = get_user_input() {
        match result {
            Ok(str) => {
                if str == "close" {
                    break;
                }

                let operation = match str.to_lowercase().as_str() {
                    "reboot" => Some(Ops::Reboot),
                    "hibernate" => Some(Ops::Hibernate),
                    "off" => Some(Ops::Off),
                    "shutdown" => Some(Ops::Shutdown),
                    "sleep" => Some(Ops::Sleep),
                    _ => None,
                };
                match operation {
                    Some(ops) => ops.take_action(),
                    None => println!("Invalid Operation"),
                }
            }
            Err(_e) => println!("Something went Wrong"),
        }
    }
}
