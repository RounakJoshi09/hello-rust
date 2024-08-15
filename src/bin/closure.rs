fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];

    let mut find_sum = |overhead: i32| {
        let mut sum = 0;
        for num in &nums {
            sum += num;
        }
        nums.pop();
        return sum + overhead;
    };

    println!("{:?}", find_sum(60));

    let sum = find_sum(50);

    println!("{:?}", sum);
}
