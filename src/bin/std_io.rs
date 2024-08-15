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
