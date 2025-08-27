// CustomSmartPointer as a data structure that stores a String
struct CustomSmartPointer {
    data: String,
}

// Implements the Drop trait to customize the behavior when an instance goes out of scope.
// This will print a message to the console to let us know when the instance is being dropped..
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // Create an instance of CustomSmartPointer and fills it with "some data".
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    // Prints that it was created
    println!("CustomSmartPointer created.");

    // Explicitly call drop to see the message before the end of main
    drop(c);

    // Prints that it was dropped
    println!("CustomSmartPointer dropped before the end of main.");
}