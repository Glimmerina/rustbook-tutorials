// Enum contains the variants Ok and Err
// Okay returns a value of type T, Err returns a value of type Exs
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// The Rust Book uses all of these in varying examples.
// However calling all of them results in errors. Do not use concurrently!!
use std::io::ErrorKind;
use std::fs::File;
use std::fs;
use std::io;
use std::io::{self, Read};
use std::error::Error;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
fn file_open() {
    let greeting_file_result = File::open("hello.txt");

    // The wording in the book is a bit confusing, but it seems to be
    // suggesting that we should handle the Result using pattern matching.
    // If the file is found, we use it; if not, we create it.
    // If there is another error, we panic.
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}