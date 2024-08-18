// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

use std::thread;

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    let hello_thread = thread::spawn(move || {
        return msg_hello();
    });

    let hello_message = thread::spawn(move || {
        return msg_thread();
    });

    let exclamation = thread::spawn(move || {
        return msg_excited();
    });

    let msg1 = hello_message.join().expect("failed to join thread 1");
    let msg2 = exclamation.join().expect("failed to join thread 2");
    let msg3 = hello_thread.join().expect("failed to join thread 3");

    println!("This is the message from thread {} {} {}", msg3, msg1, msg2);
}
