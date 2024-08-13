struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn print_coordinate(&self) {
        print!("x:{0} y:{1}", self.x, self.y);
    }

    fn move_forward(&self) -> Self {
        return Self {
            x: self.x + 1,
            y: self.y,
        };
    }

    fn move_backward(&self) -> Self {
        return Self {
            x: self.x - 1,
            y: self.y,
        };
    }
}

fn main() {
    let mut my_location: Location = Location { x: 10, y: 20 };

    my_location = my_location.move_forward();

    my_location = my_location.move_backward();

    my_location.print_coordinate();
}
