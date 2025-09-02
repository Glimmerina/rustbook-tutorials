//Allows debugging to work with Structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//Test implementation for creating methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
}
}
fn main() {
    //Creates 3 instances of Rectangle
    let rect1= Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    //Tests the can_hold method
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    //Tests the area method
    println!(
        "The area of rectangle 1 is {} square pixels.",
        rect1.area()
    );
}


