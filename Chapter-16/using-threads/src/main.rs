// Uses threads.
// Threads are used to run code concurrently.
// The rustbook claims that Rust makes it fearless to use threads.
// I am still very scared. :D
use std::thread;

fn main() {
    // Creates a vector with three elements
    let v = vec![1, 2, 3];

    // Spawns a new thread
    // The move keyword is used to move the ownership of v into the closure
    let handle = thread::spawn(move || {
        // If this prints the vector, then the ownership was successfully moved
        println!("Here's a vector: {v:?}");
    });

    // Now the main thread waits for the spawned thread to finish and then unwraps.
    handle.join().unwrap();
}
