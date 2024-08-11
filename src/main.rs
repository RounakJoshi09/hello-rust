fn add(op1: i32, op2: i32) -> i32 {
    return op1 + op2;
}

fn main() {
    println!("Hello, world!");
    let my_half = 0.5;
    let sample_int = 5;

    let first_name = "rounak";
    let last_name = "joshi";
    println!("{first_name} {last_name:?} {:?} {:?}", my_half, sample_int);

    let var1 = 10;
    let var2 = 50;
    let sum_var = add(var1, var2);
    println!("{}", sum_var);
    let sum_var_2 = add(sum_var, var2);
    println!("{}", sum_var_2);

    let mut a = 0;
    let str = "string";
    loop {
        if a == 5 {
            println!("{str} {:?}", a);
            break;
        }
        println!("{:?}", a);
        a = a + 1;
    }

    while a != 10 {
        println!("Inside While {:?}", a);
        a = a + 1;
    }
}
