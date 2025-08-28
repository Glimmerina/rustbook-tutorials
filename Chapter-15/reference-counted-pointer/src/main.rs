// Use the List crate for ref-counted lists
use crate::List::{Cons, Nil};
// Import the Rc type from the standard library.
// This is a reference-counted smart pointer that enables multiple ownership.
use std::rc::Rc;

// Creates an enum to represent a list.
enum List {
    Cons(i32, Rc<List>),
    Nil,
}


fn main() {
    // Create a new reference-counted list.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // Print the reference count of the list.
    println!("count after creating a = {}", Rc::strong_count(&a));
    // Create a new list that shares ownership of the first list.
    let b = Cons(3, Rc::clone(&a));
    // Print the reference count of the list.
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        // Create another list that shares ownership of the first list.
        let c = Cons(4, Rc::clone(&a));
        // Print the reference count of the list.
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    // Print the reference count of the list after c goes out of scope.
    // c is dropped here as the brackets close.
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
