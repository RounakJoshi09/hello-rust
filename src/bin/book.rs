use std::fmt::Display;

enum MyType {
    Type1,
    Type2,
}

fn return_item<MyType: Display, U: Display + PartialOrd>(item: MyType, num1: U, num2: U) -> MyType {
    println!(
        "Here is your item. {} , and the comparison is {}",
        item,
        num1 > num2
    );
    item
}

fn check_result(num: i32) -> Result<i32, String> {
    if num > 10 {
        Ok(num)
    } else {
        Err(format!("This number is not greater than 10: {}", num))
    }
}

fn main() {
    let item = return_item("Rounak", 9, 8);
    let item_type = MyType::Type1;
    println!("After Printing: {}", item);
}
