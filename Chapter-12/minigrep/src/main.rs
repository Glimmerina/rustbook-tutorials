// Brings the `env` module from the standard library into scope.
// We specifically needs the args bit right now.
// This allows us to access command line arguments passed to the program.
use std::env;

fn main() {
    // So it creates a vector of strings from the command line arguments.
    // The first argument is the file name, so we start from index 1.
    // The second argument is the file path we want to search in.
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    // At the moment, we are just printing the query and file path. It doesn't actually search yet.
    println!("Searching for {query}");
    println!("In file {file_path}");
}