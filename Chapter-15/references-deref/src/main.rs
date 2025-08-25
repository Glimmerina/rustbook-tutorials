// Use the Deref trait to customise the dereferencing operator
use std::ops::Deref;

fn main() {
    // x is an integer, y is a pointer to an integer
    let x = 5;
    let y = Box::new(x);

    // Assert that dereferencing y gives us the value of x
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Create a MyBox containing a String
    let m = MyBox::new(String::from("Rust"));

    // Call the hello function with a dereferenced MyBox
    hello(&(*m)[..]);
}

// Define a generic MyBox type that holds a value of type T.
// I think this is the first struct we've used with a Type T parameter, they usually have defined contents.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implements the type constructor for MyBox.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Prints a greeting to the provided name when called.
fn hello(name: &str) {
    println!("Hello, {name}!");
}