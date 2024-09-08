use std::{iter::Rev, thread};

use crossbeam::channel::{unbounded, SendError, Sender};

enum ThreadMessage {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}
enum MainMessage {
    Sum(i64),
    MainQuit,
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

    let (main_sender, main_reciever) = unbounded();

    let handle_thread = thread::spawn(move || loop {
        match reciver.recv() {
            Ok(thread_msg) => match thread_msg {
                ThreadMessage::PrintData(msg) => println!("{}", msg),
                ThreadMessage::Sum(x, y) => {
                    println!("Summing...........");
                    main_sender.send(MainMessage::Sum((x + y)));
                }
                ThreadMessage::Quit => {
                    println!("Terminating thread");
                    main_sender.send(MainMessage::MainQuit);
                }
            },
            Err(e) => {
                println!("Disconnected");
                break;
            }
        }
    });

    sendMessages(sender);

    while let Ok(msg) = main_reciever.recv() {
        match msg {
            MainMessage::Sum(sum) => println!("Sum is {}", sum),
            MainMessage::MainQuit => println!("Main Terminated"),
        }
    }

    handle_thread.join();
}
