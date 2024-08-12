enum Color {
    Red,
}

fn checking_tuple() -> (i32, i32, i32) {
    (1, 2, 3)
}

fn getColourName(color: Color) -> String {
    match color {
        Color::Red => return "Red".to_owned(),
    }
}

fn main() {
    let (number, color) = (1, Color::Red);
    let tuple_numbers = checking_tuple();
    let (one, two, three) = checking_tuple();

    println!("The numbers are {:?} {:?} {:?}", one, two, three);
    println!(
        "The tuple numbers are {:?} {:?} {:?}",
        tuple_numbers.0, tuple_numbers.1, tuple_numbers.2
    );
    println!("The Color is {:?}", getColourName(color))
}
