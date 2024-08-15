// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    name: String,
    age: i32,
}

impl Customer {
    fn can_make_purchase(&self) -> Result<bool, String> {
        if self.age > 21 {
            return Ok(true);
        } else {
            return Err("Age is less than 21 therefore cannot make purchase".to_owned());
        }
    }

    fn check_purchase(&self) -> Result<(), String> {
        let status = self.can_make_purchase()?;
        if status == true {
            println!("We can make the purchase")
        }

        return Ok(());
    }
}

fn main() {
    let customer1 = Customer {
        age: 10,
        name: "Rounak".to_owned(),
    };

    let result = customer1.check_purchase();

    match result {
        Err(str) => println!("{}", str),
        _ => (),
    }
}
