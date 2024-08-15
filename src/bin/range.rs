fn main() {
    let rng = 1..5;
    for num in rng {
        println!("{:?}", num);
    }

    let char_rng = 'a'..='z';
    for chr in char_rng {
        println!("{:?}", chr);
    }
}
