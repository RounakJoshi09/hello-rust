enum Direction {
    Up,
    Down,
    Left,
    Right,
    One = 10,
}
fn main() {
    let direction: Direction = Direction::Up;
    match direction {
        Direction::Up => println!("This is Up Direction"),
        Direction::Down => println!("This is Down Direction"),
        Direction::Left => println!("This is Left Direction"),
        _ => println!("This is some other direction"),
    }
}
