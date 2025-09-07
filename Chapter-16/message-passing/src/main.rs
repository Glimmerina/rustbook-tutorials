// Uses sync, thread and time.
// These are used to create threads and pause execution for a specified duration.
// The mpsc module is used to create a channel for message passing between threads.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel for communication between threads.
    let (tx, rx) = mpsc::channel();

    // Clone the transmitter to allow multiple threads to send messages.
    let tx1 = tx.clone();
    // Spawn the first thread to send messages with a vector of strings.
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        // Iterate over the vector and send each string through the channel with a delay.
        // The delay is to ensure they are sent one at a time, and in the right order.
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Spawn the second thread to send another set of messages.
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        // Once again iterates with a delay
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // For each received message from the channel, print it to the console.
    // This is a horrifically inefficient way to do this, but it works.
    // And besides who am I to question the Rustbook? In Crab we trust.
    for received in rx {
        println!("Got: {received}");
    }
}