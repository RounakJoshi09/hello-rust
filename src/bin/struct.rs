struct UserInfo {
    name: String,
    age: i32,
    balance: f64,
}
fn main() {
    let user1: UserInfo = UserInfo {
        age: 30,
        balance: 163.5,
        name: "Rounak Joshi".to_string(),
    };

    println!(
        "The name of the user is {0} , age is {1} and the account balance {2}",
        user1.name, user1.age, user1.balance
    )
}
