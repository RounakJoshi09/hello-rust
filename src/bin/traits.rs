trait Mensuration {
    fn perimeter(&self) -> f64;
}

struct Square {
    side: f64,
}

struct Triangle {
    side1: f64,
    side2: f64,
    side3: f64,
}

impl Mensuration for Square {
    fn perimeter(&self) -> f64 {
        return self.side * 4.0;
    }
}

impl Mensuration for Triangle {
    fn perimeter(&self) -> f64 {
        return self.side1 + self.side2 + self.side3;
    }
}

fn check_box_fit(shape: impl Mensuration) -> bool {
    let perimeter = shape.perimeter();
    if perimeter > 250.0 {
        return false;
    } else {
        return true;
    }
}

fn main() {
    let square = Square { side: 40.3 };
    let triangle = Triangle {
        side1: 10.2,
        side2: 20.3,
        side3: 30.0,
    };

    let can_triangle_fit = check_box_fit(triangle);
    let can_square_fit = check_box_fit(square);

    if can_triangle_fit == true {
        println!("Triangle can fit into the box")
    }

    if can_square_fit == true {
        println!("Square can fit into the box")
    }
}
