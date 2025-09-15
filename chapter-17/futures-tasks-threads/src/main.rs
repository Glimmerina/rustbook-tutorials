use std::{thread, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    // creates a channel with a sender and receiver
    let (tx, mut rx) = trpl::channel();

    // Spawns a thread to send messages
    // After each message, the thread sleeps for 1 second
    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Spawns a task to receive messages and print them
    // The task runs until the channel is closed and all messages are received

    trpl::run(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}

// Unused function that demonstrates how to create a stream of intervals
// It spawns a thread that sends an incrementing count every millisecond
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    // The tutorial wanted us to use the standard library and not trpl.
    // But it didn't work unless we used `trpl::thread` instead of `std::thread`.
    // Did something change or did I get it wrong? 
    // Idk, I'm tired, I've been at work all day. I'm using trpl.
    // trpl was there for me. It didn't let me down. In trpl I trust.

    thread::spawn(move || {
        let mut count = 0;
        loop {
            // As before, this was meant to use standard but standard didn't work.
            // I'm vegan, btw. Just thought you should know.
            thread::sleep(Duration::from_millis(1));
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}