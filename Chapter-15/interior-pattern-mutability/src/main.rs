#[derive(Debug)]

// Enum List that holds a reference-counted pointer to a RefCell containing an i32
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

// Uses the list crate for reference counting and interior mutability
use crate::List::{Cons, Nil};
// Standard library imports for reference counting and interior mutability
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Create a reference-counted RefCell containing the value 5
    let value = Rc::new(RefCell::new(5));

    // Create a list 'a' that holds the value and points to Nil
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // Create another list 'b' that holds the value 3 and points to 'a'
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    // Create another list 'c' that holds the value 4 and points to 'a'
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // *value.borrow_mut will change the value inside the RefCell
    // This change will be reflected in both lists 'b' and 'c' since they share the same Rc<RefCell<i32>>
    *value.borrow_mut() += 10;

    // Print the lists to see the updated values
    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}