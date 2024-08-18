// Here we are wrapping the closure with box, because closure can be of any size, but to call a function
// the function parameters should be of defined size, now Box is a pointer which is of defined size, its
//just ki it is pointing to some undefiend size memory of closure in the heap
fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    return 0;
}
fn main() {
    let name = "Rounak";

    // move is used to move the outer variable in the heap in order to use it with closure
    // since closure stores on heap
    let add: Box<_> = Box::new(move |a: i32, b: i32| -> i32 {
        println!("Doing operation for {}", name);
        return a + b;
    });

    math(50, 100, add);
}
