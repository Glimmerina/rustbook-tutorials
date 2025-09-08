// Uses Arc and Mutex to safely share and modify a counter across multiple threads.
// This is because Arc and Mutex are used for thread-safe reference counting and mutual exclusion.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a counter protected by a Mutex and shared via an Arc.
    // The Mutex ensures that only one thread can modify the counter at a time.
    // The Arc allows multiple threads to own the counter.
    let counter = Arc::new(Mutex::new(0));
    // Vector to hold the thread handles.
    let mut handles = vec![];

    // Spawn 10 threads to increment the counter.
    for _ in 0..10 {
        // Clone the Arc to get a new reference for the thread.
        let counter = Arc::clone(&counter);
        // Spawn a new thread.
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        // The Handle is used to push the thread into the vector.
        handles.push(handle);
    }

    // For every handle in the vector, we join the thread.
    // This ensures that the main thread waits for all spawned threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value of the counter.
    println!("Result: {}", *counter.lock().unwrap());
}