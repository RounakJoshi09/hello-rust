use std::collections::HashMap;

#[derive(Debug)]
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
    let mut location_map = HashMap::new();

    location_map.insert("rounak", Location { x: 10, y: 12 });
    location_map.insert("ram", Location { x: 15, y: 15 });
    location_map.insert("krsna", Location { x: 15, y: 15 });
    location_map.insert("srila prabhupad", Location { x: 13, y: 13 });

    let krsna_location = location_map.get("krsna");

    match krsna_location {
        Some(location) => println!("x:{}, y:{}", location.x, location.y),
        None => println!("Location not found"),
    }

    for (name, location) in location_map.iter() {
        println!("Name : {}", name);
        location.print_coordinate();
    }

    for name in location_map.keys() {
        println!("Name : {}", name);
    }

    for location in location_map.values() {
        location.print_coordinate();
    }
}
