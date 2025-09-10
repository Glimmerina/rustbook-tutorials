// Uses time. When do we not use time? 
// It's the start of a tutorial, it's too early to get existential.

use std::time::Duration;
use trpl;

fn main() {
// Create the runtime.
    trpl::run(async {
        // Create a channel with a sender and receiver.
        // This is a standard method of sending and receiving messages between code blocks.
        // Second Life flashbacks. LSL Nightmares. Aaaaaaaa.
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        // Create a future that sends messages.
        // But how does the future send messages and not presents. 
        // If you are reading this, I am in a weird mood right now. Forgive my attempts at comedy.
        // Unless you are future me. Then you ARE in the future. Did the messages send?
        let tx1_fut = async move {

            // Create a vector of strings to send.
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            // Send each string in the vector, pausing for 500 milliseconds between each send.
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // Create a future that receives messages.
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        // Create another future that sends messages.
        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            // Send each string in the vector, pausing for 1500 milliseconds between each send.
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        // Run all the futures concurrently.
        // The receiver future will run until all senders are dropped and all messages are received.
        // This is a common pattern in asynchronous programming to ensure that all tasks are completed.
        trpl::join3(tx1_fut, tx_fut, rx_fut).await;
    });
}