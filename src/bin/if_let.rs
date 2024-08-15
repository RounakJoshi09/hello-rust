enum Color {
    Red,
    Green,
    Purple,
}
fn get_card() -> Option<i32> {
    return Some(40);
}

fn main() {
    if let Some(num) = get_card() {
        println!("This is the num : {:?}", num);
    } else {
        println!("Num not found");
    }

    let color = Color::Purple;

    if let Color::Purple = color {
        println!("This is purple color")
    } else {
        println!("I dont know this color")
    }
}
