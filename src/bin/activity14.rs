// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    fav_color: String,
}

impl Person {
    fn print_name_color(&self) {
        println!("name: {:?} color: {:?}", self.name, self.fav_color)
    }
}

fn main() {
    let list = vec![Person {
        age: 30,
        fav_color: "Red".to_owned(),
        name: String::from("Rounak"),
    }];

    for l in list {
        l.print_name_color();
        println!("{:?}", l.age)
    }
}
