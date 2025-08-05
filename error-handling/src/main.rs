enum Result<T, E> {
    Ok(T),
    Err(E),
}

use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
