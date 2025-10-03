// Calls the procedural macro defined in the `hello_macro` crate.
// This example assumes that the `hello_macro` crate is included in Cargo.toml. We added it but it still errors. Why.
use hello_macro::HelloMacro;

// Define a struct named `Pancakes`.
struct Pancakes;

// When the macro is implemented for `Pancakes`, it will call a function that prints a message.
impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

// Executes the macro on the pancakes function.
fn main() {
    Pancakes::hello_macro();
}