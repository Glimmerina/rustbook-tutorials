// Uses the standard library for concurrency.
use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

// Creates a struct for the threadpool.
// It consists of workers that will execute the jobs sent to the pool.
// It also includes a sender to send jobs to the workers.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

// Creates a struct for Jobs.
struct Job;

// Creates a struct for workers that includes ID and a thread.
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

// Implements the worker struct. 
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // Spawns a new thread for the worker.
        let thread = thread::spawn(move || {
            // The worker thread will loop forever, waiting for jobs to execute.
            loop {
                let message = receiver.lock().unwrap().recv();

                // When a job is received, execute it.
                // If the channel is closed, break the loop and end the thread.
                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}

// Implementation of the ThreadPool struct.
// The tutorial had me remake this code block so many times. 
// So many comments had to be purged. So many. Comment exterminatus.
impl ThreadPool {

    // Creates a new thread pool with a given size. Asserts that the size is greater than 0.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Creates a channel for sending and receiving jobs.
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        // Creates a vector to hold the workers. It's mutable and has a capacity of the given size.

        let mut workers = Vec::with_capacity(size);

        // For each job id, creates a new worker and adds it to the workers vector.
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    // Executes a job by sending it.

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

// Implements the Drop trait for the ThreadPool struct to gracefully shut down the pool.
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}

