// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn main() {
    let name = "sam";

    // let make_user = |id| -> User {
    //     return User {
    //         user_id: id,
    //         name: name.to_owned(),
    //     };
    // };

    let user = find_user(name).map(|id| User {
        name: name.to_owned(),
        user_id: id,
    });

    // let user = find_user(name).map(|id| -> User { return make_user(id) });

    match user {
        Some(temp_user) => println!("User Found : {:?}", temp_user),
        None => println!("User Not Found"),
    }
}
