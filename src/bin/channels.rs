use std::{iter::Rev, thread};

use crossbeam::channel::{unbounded, SendError, Sender};

enum ThreadMessage {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

fn sendMessages(sender: Sender<ThreadMessage>) -> Result<(), SendError<ThreadMessage>> {
    sender.send(ThreadMessage::PrintData("Printed From threads".to_owned()))?;
    sender.send(ThreadMessage::Sum(10, 90))?;
    sender.send(ThreadMessage::Quit)?;
    // drop(sender);
    return Ok(());
}

fn main() {
    let (sender, reciver) = unbounded();

    let reciever2 = reciver.clone();

    let handle_thread = thread::spawn(move || loop {
        match reciver.recv() {
            Ok(thread_msg) => match thread_msg {
                ThreadMessage::PrintData(msg) => println!("{}", msg),
                ThreadMessage::Sum(x, y) => println!("Sum of {} and {} is {}", x, y, (x + y)),
                ThreadMessage::Quit => {
                    println!("Terminating thread");
                    break;
                }
            },
            Err(e) => {
                println!("Disconnected");
                break;
            }
        }
    });

    let res = sendMessages(sender);

    match res {
        Ok(_) => (),
        Err(msg) => println!("Something went Wrong {}", msg),
    }

    handle_thread.join();
}
