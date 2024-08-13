struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("{:?}", name);
}

fn main() {
    let reciept = vec![
        LineItem {
            count: 30,
            name: "Cerials".to_owned(),
        },
        LineItem {
            count: 10,
            name: String::from("Fruits"),
        },
    ];

    for rec in &reciept {
        print_name(&rec.name);
        println!("{:?}", rec.count);
    }

    for rec in reciept {
        print_name(&rec.name);
        println!("{:?}", rec.count);
    }
}
