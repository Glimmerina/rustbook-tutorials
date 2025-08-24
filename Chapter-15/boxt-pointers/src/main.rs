use crate::List::{Cons, Nil};

fn main() {
    // The box is allocated on the heap.
    // Most of the time we won't need a box for a single value, the stack would be more appropriate.
    let b = Box::new(5);
    println!("b = {b}");

    // Boxes are often used in recursive types.
    // This one is a cons list, where each element points to the next.
    let list = Cons(1, Cons(2, Cons(3, Nil)));

    // This version works because we are using boxes to store the next element in the list.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}


enum List {

    // This list returns an error because it has infinite size. Oops.
    //Cons(i32, List),

    // By using a box, we are storing a pointer to the next element in the list,
    // which has a known size (the size of a pointer) at compile time.
    Cons(i32, Box<List>),
    Nil,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
