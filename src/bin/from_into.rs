struct Uppercase(String);

impl From<String> for Uppercase {
    fn from(value: String) -> Self {
        return Uppercase(value.to_uppercase());
    }
}

impl From<&str> for Uppercase {
    fn from(value: &str) -> Self {
        return Uppercase(value.to_uppercase());
    }
}

fn main() {
    let uppercase = Uppercase::from("lowercase");

    let uppercase2: Uppercase = "rounak".into();

    println!(
        "The string is Uppercase :{:?} , Uppercase2: {:?}",
        uppercase.0, uppercase2.0
    );
}
