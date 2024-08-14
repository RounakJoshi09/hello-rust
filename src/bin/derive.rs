#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn print_direction(direction: Direction) {
    println!("{:?}", direction);
}

fn main() {
    let direction = Direction::Left;
    print_direction(direction);
    print_direction(direction);
}
