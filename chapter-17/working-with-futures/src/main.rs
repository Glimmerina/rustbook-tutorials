// Uses the trpl crate and the pin, future and time modules from the standard library.
// They are used to handle asynchronous operations and timeouts.
use trpl::Either;
use std::pin::{Pin, pin};
use std::future::Future;
use std::time::Duration;


fn main() {
    // Run the asynchronous code using trpl's runtime.
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        // Attempt to run the slow future with a timeout of 2 seconds.
        // If it completes in time, print the result; otherwise, print a timeout message.
        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

//fn slow(name: &str, ms: u64) {
  //  thread::sleep(Duration::from_millis(ms));
    //println!("'{name}' ran for {ms}ms");
//}
// This function takes a future and a maximum time duration.
// It returns a Result that is Ok if the future completes in time, or Err if it times out.
async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}