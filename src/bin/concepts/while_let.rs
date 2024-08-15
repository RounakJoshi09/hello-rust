fn main() {
    let mut data = Some(50);

    let mut i = 0;

    while let Some(num) = data {
        println!("{:?}", num);
        if i == 4 {
            data = None;
        }
        i = i + 1;
    }

    let nums = vec![1, 2, 3, 4, 5];

    let mut nums_itr = nums.iter();

    while let Some(num) = nums_itr.next() {
        println!("Number - {:?}", num);
    }
}
