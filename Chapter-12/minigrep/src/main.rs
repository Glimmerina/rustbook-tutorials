// Brings the `env` module from the standard library into scope.
// We specifically needs the args bit right now.
// This allows us to access command line arguments passed to the program.
use std::env;

// Brings the `fs` module from the standard library into scope.
// This module provides functionality for working with the file system!

use std::fs;

// Brings the `process` module from the standard library into scope.
// This module provides functionality for handling processes, including exiting the program.
use std::process;

// Brings the `Error` trait into scope.
// This trait is used for error handling in Rust, allowing us to define custom error types.
use std::error::Error;

// Uses the search function from the minigrep module.
use minigrep::{search, search_case_insensitive};

// Defines a struct named `Config` to store the contents of the search and file path.
// As of 12.5, now also stores whether to ignore case sensitivity.
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
fn main() {
    // So it creates a vector of strings from the command line arguments.
    // The first argument is the file name, so we start from index 1.
    // The second argument is the file path we want to search in.
    let args: Vec<String> = env::args().collect();

    // Calls a struct to store the contents and file path.
    // 13.3 onward: now calls an iterator to build the `Config` struct.
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Prints the search query and file path to the console.
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // If the `run` function returns an error, it will print the error message and exit the program with a non-zero status code.
    // If it succeeds, it will continue running the program.
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    //run(config);
}

// Reads the contents of the file specified by `file_path`.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// The new version of this from Chapter 13.3.
// The chapter said this should be in src/lib.rs, but we built it here and it errors if we move it.

// The new version of the `Config` struct.
// It now has a `build` method that takes an iterator of strings as input.
impl Config {
    pub fn build(
        // Takes an iterator of strings as input
        // `args` is an iterator that yields String items.
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        // Extracts the query string and file path from the iterator.
        // If either is missing, it returns an error.
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        // Extracts the file path from the iterator.
        // If the file path is missing, it returns an error.
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // Checks if the `IGNORE_CASE` environment variable is set.
        // If it is set, it will ignore case sensitivity in the search.
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // Returns a `Config` instance with the query, file path, and ignore_case flag.
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
