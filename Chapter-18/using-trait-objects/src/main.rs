// Calls the gui library. Or at least it would, it doesn't exist. This is a theoretical for the chapter.
// Otherwise known as a Graphical User Interface.
// You do not pronounce it as Jraphical User Interface because that would be silly.
// I'm still salty about this. Bloody GIF.

use gui::Draw;

// Makes a struct called SelectBox with three fields: width, height, and options.
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

// Implements the Draw trait for the SelectBox struct.
// When executed, the function will draw on itself.
impl Draw for SelectBox {
    fn draw(&self) {
        
    }
}

// Calls the Button and Screen parts from the gui library.
use gui::{Button, Screen};

// When ran, this defines a screen variable that is an instance of the Screen struct.
// It then populates the components vector with a SelectBox and a Button.
// Finally, it calls the run method on the screen variable which will in turn call the draw
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}