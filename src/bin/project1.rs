// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::io;

struct Bill {
    name: String,
    amount: i32,
}

impl Bill {
    fn new(name: String, amount: i32) -> Self {
        Bill { name, amount }
    }

    fn print_bill(&self) {
        println!("------------------------------------");
        println!("Name : {}", self.name);
        println!("Amount : {}", self.amount);
        println!("------------------------------------");
    }

    fn generate_new_bill() -> Bill {
        let mut name = String::new();
        println!("Enter name of the bill : ");
        io::stdin().read_line(&mut name);

        let mut amount = String::new();
        println!("Enter amount of the bill (in integer) : ");

        let mut parsed_amount = 0;

        while let Ok(_) = io::stdin().read_line(&mut amount) {
            let is_decimal = match amount.trim().parse::<i32>() {
                Ok(amt) => {
                    parsed_amount = amt;
                    true
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    false
                }
            };

            if is_decimal {
                break;
            } else {
                println!("Invalid Input");
            }
            amount.clear(); // Clear the input buffer for the next input
        }
        return Bill::new(name, parsed_amount);
    }
}

fn view_bill(bills: &Vec<Bill>) {
    for bill in bills {
        bill.print_bill();
    }
}

fn remove_bill(bills: &mut Vec<Bill>) {
    println!("Enter bill name to be removed : ");

    let mut bill_name = String::new();
    let result = io::stdin().read_line(&mut bill_name);

    if result.is_ok() {
        let bill = bills
            .iter()
            .position(|bill| bill.name.to_lowercase() == bill_name.to_lowercase());
        if bill.is_some() {
            bills.remove(bill.unwrap_or_else(|| 0));
            println!("* Bill Removed Successfully *");
            println!("")
        } else {
            println!("Bill with name :{:?} not found, Try Again", bill_name)
        }
    }
}

fn edit_bill(bills: &mut Vec<Bill>) {
    println!("Enter bill name you want to edit : ");

    let mut bill_name = String::new();
    let result = io::stdin().read_line(&mut bill_name);

    if result.is_ok() {
        let bill = bills
            .iter()
            .position(|bill| bill.name.to_lowercase() == bill_name.to_lowercase());

        if bill.is_some() {
            let index = bill.unwrap_or_default();
            println!("Info of the bill you want to edit ");
            bills[index].print_bill();

            println!("Enter Details of the new bill");

            let new_bill = Bill::generate_new_bill();

            bills[index] = new_bill;

            println!("* Bill Editted Successfully *");
            println!("")
        } else {
            println!("Bill with name :{:?} not found, Try Again", bill_name)
        }
    }
}

fn show_info() {
    println!("Welcome to Billing Calc");
    println!("1. Add Bill");
    println!("2. View Bill");
    println!("3. Remove Bill");
    println!("4. Edit Bill");
    println!("5. Close");
}

fn main() {
    let mut bills: Vec<Bill> = Vec::new();

    show_info();

    let mut operation = String::new();

    while let Ok(_) = io::stdin().read_line(&mut operation) {
        match operation.as_str().trim() {
            "1" => {
                let bill = Bill::generate_new_bill();
                bills.push(bill);
                println!("** Bill Added Successfully **");
                show_info();
            }
            "2" => {
                view_bill(&bills);
                show_info();
            }
            "3" => {
                remove_bill(&mut bills);
                show_info();
            }
            "4" => {
                edit_bill(&mut bills);
                show_info();
            }
            "5" => {
                break;
            }
            _ => (),
        }
        operation.clear()
    }
}
