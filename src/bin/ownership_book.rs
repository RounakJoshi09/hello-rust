struct User {
    user_name: String,
    age: i32,
}

fn print_name(user: &User) {
    println!("{}", user.user_name)
}

fn print_age(user: &User) {
    println!("{}", user.age)
}

fn main() {
    let user = User {
        age: 30,
        user_name: "Rounak Joshi".to_string(),
    };

    print_name(&user);
    print_age(&user);
}
