// Brings the `env` module from the standard library into scope.
// We specifically needs the args bit right now.
// This allows us to access command line arguments passed to the program.
use std::env;

// Brings the `fs` module from the standard library into scope.
// This module provides functionality for working with the file system!

use std::fs;
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

    // Reads the contents of the file specified by `file_path`.
    // If the file cannot be read, it will panic with the message provided.
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // Prints the contents of the file to the console.
    println!("With text:\n{contents}");
}

