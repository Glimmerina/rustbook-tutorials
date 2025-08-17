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
use minigrep::search;

// Defines a struct named `Config` to store the contents of the search and file path.
struct Config {
    query: String,
    file_path: String,
}
fn main() {
    // So it creates a vector of strings from the command line arguments.
    // The first argument is the file name, so we start from index 1.
    // The second argument is the file path we want to search in.
    let args: Vec<String> = env::args().collect();

    // Calls a struct to store the contents and file path.
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Prints the search query and file path to the console.
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // If the `run` function returns an error, it will print the error message and exit the program with a non-zero status code.
    // If it succeeds, it will continue running the program.
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    //run(config);
}

// Reads the contents of the file specified by `file_path`.
// If the file cannot be read, it will panic with the message provided.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
