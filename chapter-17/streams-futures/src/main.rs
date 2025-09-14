// Use std library's pin and time modules to manage stream pinning and timeouts
use std::{pin::pin, time::Duration};
// Use trpl receiver stream and stream extensions for async stream handling.
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    // Run the async block using trpl's runtime
    trpl::run(async {
        let mut messages =
            // Pin the stream and set a timeout of 200 milliseconds for each item
            pin!(get_messages().timeout(Duration::from_millis(200)));

            // While it is running, process each message or handle timeout errors
        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                // I keep picturing Trollface appearing to ask "problem?" and mock me.
                // Literally brain poisoned.
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}

// Function to simulate getting messages asynchronously
fn get_messages() -> impl Stream<Item = String> {
    // Create a channel for sending and receiving messages
    let (tx, rx) = trpl::channel();

    // Spawn a new asynchronous task to send messages
    trpl::spawn_task(async move {
        // Sends a bunch of letters through the alphabet
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        // For each message, sleep for a bit and then send it through the channel
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            // If sending fails, log the error and break the loop
            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    // Return a stream that receives messages from the channel
    ReceiverStream::new(rx)
}

// Function to simulate getting intervals asynchronously.
// The tutorial told me to make this and then never calls it. But here it is.
// Its very similar to the previous function but instead of sending letters, it counts intervals.
fn get_intervals() -> impl Stream<Item = u32> {
    // Create a channel for sending and receiving interval counts
    let (tx, rx) = trpl::channel();

    // Spawn a new asynchronous task to send interval counts
    // This simulates a timer that ticks every millisecond
    trpl::spawn_task(async move {
        // For each tick, sleep for 1 millisecond and then send the count through the channel
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            // If sending fails, log the error and break the loop
            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}