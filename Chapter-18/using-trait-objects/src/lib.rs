// Makes a public trait called draw.
// At the moment it calls a function called draw that takes an immutable reference to self and returns nothing.
pub trait Draw {
    fn draw(&self);
}

// Makes a public struct called Screen that has a generic type parameter T.
// The struct has a single field called components that is a vector of the generic type T.
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

// Implements methods on the Screen struct for any type T that implements the Draw trait.
impl<T> Screen<T>
where
    T: Draw,
{
    // A public method called run that goes through each component in the components vector and calls its draw method.
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Makes a public struct called Button with three fields: width, height, and label.
// We cannot click the button. Not yet. But we will.
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// Implements the Draw trait for the Button struct.
// We needed all this code to draw a button. I could have done it with a crayon in seconds.
// I don't think Rust accepts crayon drawings though. Yet.
impl Draw for Button {
    fn draw(&self) {
    
    }
}