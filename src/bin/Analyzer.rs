use std::io::stdin;

enum UserInput {
    Number(i32),
    Mean,
    Min,
    Max,
    Quit,
    InvalidInput,
}

fn print_instructions() {
    println!("You can enter any 'number','mean','min','max' or 'q' to exit the program.");
}

fn take_user_input() -> Result<UserInput, UserInput> {
    let mut input = String::new();
    let user_input = stdin().read_line(&mut input);

    if user_input.is_err() {
        println!("There was an error");
        return Ok(UserInput::InvalidInput);
    }

    match input.trim().to_lowercase().as_ref() {
        "q" => return Err(UserInput::Quit),
        "mean" => return Ok(UserInput::Mean),
        "min" => return Ok(UserInput::Min),
        "max" => return Ok(UserInput::Max),
        _ => {
            match input.trim().parse::<i32>() {
                Ok(num) => return Ok(UserInput::Number(num)),
                Err(_) => return Ok(UserInput::InvalidInput),
            };
        }
    }
}
fn main() {
    println!("Welcome to Rust Analyzer!");
    print_instructions();
    let mut number_list: Vec<i32> = Vec::new();
    while let Ok(user_input) = take_user_input() {
        match user_input {
            UserInput::InvalidInput => {
                println!("Invalid input");
                print_instructions();
                continue;
            }
            UserInput::Number(num) => {
                number_list.push(num);
            }
            UserInput::Max => {
                let Some(max) = number_list.iter().max() else {
                    println!("No numbers entered");
                    continue;
                };
                println!("Max number is {}", max);
            }
            UserInput::Min => {
                let Some(min) = number_list.iter().min() else {
                    println!("No numbers entered");
                    continue;
                };
                println!("Min number is {}", min);
            }
            UserInput::Mean => {
                if (number_list.is_empty()) {
                    println!("No numbers entered");
                }
                let mean = number_list.iter().sum::<i32>() / number_list.len() as i32;
                println!("Mean is {}", mean);
            }
            _ => {}
        }
    }
}
