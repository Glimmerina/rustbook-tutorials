// This tutorial was a bit of a mess to write out.
// I think you were supposed to read it rather than follow it in an IDE. Oh well.



use std::ops::Add;
// So traits can all have the same method name. It's inefficient but you CAN do it.
trait Pilot {
    fn fly(&self);
}

// Please note that wizards can only fly if they have a spellslot available for it.
// Storm sorcerers can fly after any spell cast. But wizards need to prepare the spell. Nerds.
trait Wizard {
    fn fly(&self);
}

struct Human;

// We can implement the same trait for different types. Each gets its own behaviour.

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}


impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
// Associated functions that are not methods will not have a Self parameter.
// This code is an elaborate way of naming every puppy "Spot". This is a bad idea.
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}


use std::fmt;
// This will only work for types that implement Display.
// If it doesn't implement Display, it won't compile. Buy a display, nerd.
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

// And now we use the wrapper.
// The Wrapper struct is a tuple struct that contains a Vec<String>.

// As a NewType, it is a separate type from Vec<String>.
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// So basically the wrapper will print the vector of strings in a nice format.
// Without the wrapper, we can't implement Display for Vec<String> directly
fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}

// Througout this tutorial we replace fn main a lot. So I kept the old ones as code for posterity.
//fn main() {
//    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
// }

//fn main() {
//    let person = Human;
//    Pilot::fly(&person);
//    Wizard::fly(&person);
//    person.fly();
// }

// fn main() {
//    assert_eq!(
//        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
//       Point { x: 3, y: 3 }
//    );
//}