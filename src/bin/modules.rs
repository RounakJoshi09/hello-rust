mod map {

    pub fn print_welcome_msg() {
        println!("Welcome to World Map v2.0");
    }
    pub struct Location {
        pub x: i32,
        pub y: i32,
    }

    impl Location {
        pub fn print_coordinate(&self) {
            print!("x:{0} y:{1}", self.x, self.y);
        }

        pub fn move_forward(&self) -> Self {
            return Self {
                x: self.x + 1,
                y: self.y,
            };
        }

        pub fn move_backward(&self) -> Self {
            return Self {
                x: self.x - 1,
                y: self.y,
            };
        }
    }
}

fn main() {
    use map::*;

    print_welcome_msg();

    let mut my_location: Location = Location { x: 10, y: 20 };

    my_location = my_location.move_forward();

    my_location = my_location.move_backward();

    my_location.print_coordinate();
}
