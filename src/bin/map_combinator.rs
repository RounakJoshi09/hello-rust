fn get_card() -> Option<i32> {
    return None;
}

fn main() {
    let card_num = match get_card() {
        Some(num) => num,
        None => 0,
    };

    let card_num_2 = get_card()
        .map(|num| -> i32 { num + 2 })
        .map(|num| -> i32 { num * 2 });

    match card_num_2 {
        Some(num) => println!("Card Num :{:?}", card_num_2),
        None => println!("Nothing is returned"),
    }
}
