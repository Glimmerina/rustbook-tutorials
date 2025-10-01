fn main() {
    // Rust lets us declare a type alias to give an existing type another name.
    // So in this case, Kilometers is a new name for i32.
    // I do not understand why you would want to do this, but it is possible.
    // Gonna rename all my types to different types of crab. Rustacean mode engaged.


    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    // Prints the values of both to prove the alias works.
    println!("x + y = {}", x + y);


    // Declares a type alias called Thunk for a boxed closure that takes no parameters.
    type Thunk = Box<dyn Fn() + Send + 'static>;

    // Here we use the Thunk type alias to declare a variable f.
    let f: Thunk = Box::new(|| println!("hi"));

    // Calls the fmt module, which is used to format text.
    // Also calls the io module, which is used for input and output. Specifically we're using it for error handling.
    use std::fmt;
    use std::io::Error;

    // Declares a trait called Write with several methods for writing data.
    // It then uses the Result type from the io module to handle potential errors.
    pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
}


// The other types in this tutorial are short form examples and not really worth writing the code for.
// Useful to know that ! is the never type though. It indicates that a function will never return.

