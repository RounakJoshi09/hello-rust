// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

use std::string;

fn add(op1: i32, op2: i32) -> i32 {
    op1 + op2
}

fn main() {
    let op1 = 10;
    let op2 = 40;

    let res = add(op1, op2);
    let ch: u8 = 32;
    println!(
        "The sum of {op1:?} and {op2:?} is {res:?} and {}",
        ch as char
    );
}
