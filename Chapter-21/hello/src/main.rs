
// Uses the io crate for input/output operations and the net crate for tcp networking. 
// *update now also uses the filesystem crate.  
// **Update now also uses the thread and time crates.
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

// Calls the threadpool from the lib.rs file.
use hello::ThreadPool;

fn main() {
    // Binds the listener to the address and port.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // Creates 4 threads in a thread pool. 
    // This limits the number of threads that can be spawned at once to 4.
    // If more than 4 connections are made, the additional connections will wait until a thread is available.
    // I wonder if this helps with DDOS attacks?
    let pool = ThreadPool::new(4);
    // For each incoming connection, handle the connection by calling the handle_connection function.
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

// A function to handle the connection.
fn handle_connection(mut stream: TcpStream) {
    // Create a buffered reader from the stream and read lines until an empty line is encountered.
    let buf_reader = BufReader::new(&stream);
    // Read the first line of the HTTP request.
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Collect the lines into a vector.
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // Print the request line and the full HTTP request to the console.
    // If it's valid, load the hello file. If not, error 404.
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // Read the contents of the file and get its length.
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    // Format the HTTP response and write it to the stream.
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}