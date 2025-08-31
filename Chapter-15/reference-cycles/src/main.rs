// Using `Rc`, `RefCell`, and `Weak` to create a tree data structure with parent pointers
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
// Define a List enum to demonstrate reference cycles with Rc and RefCell. Later replaced with the struct.
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

// Struct to represent a node in a tree with parent and children pointers.
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
// List methods to get the tail of the list.
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // Creates a leaf node with no parent and no children.
    let leaf = Rc::new(Node {
        value: 3,
        // The purpose of Weak is to avoid reference cycles.
        // This works by not increasing the reference count, which would prevent the memory from being freed.
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // At this point, leaf has no parent. The strong count is 1 (the Rc itself), and the weak count is 0.
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    // Now creates a branch node that has the leaf as a child.
    // The branch node is created inside a new scope to demonstrate that the leaf can outlive the branch.
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    // An error on this line, not addressed in the Rustbook. Dammit folks.
    // The crab god is unhappy.
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

// Originally was main, renamed it as not to lose the code.
// I don't think anyone actually reads these but I do follow these tutorials carefully and I want all the code to be here, dangit.
fn printRefList() {
    // Defines a reference-counted list with interior mutability.
    // Then prints them.
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());
}