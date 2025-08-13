// Creates a public function that adds two unsigned ints together.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Creates a public function that adds two to an unsigned int.
pub fn add_two(a: u64) -> u64 {
    a + 2
}

// Creates a public function that returns a greeting string.
pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

// Simple public i32 struct that holds a value.
pub struct Guess {
    value: i32,
}

// Derives the Debug trait for the Rectangle struct, allowing it to be printed using the {:?} format specifier.
// This is useful for debugging purposes.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Implements a constructor for the Guess struct that checks if the value is between 1 and 100.

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        Guess { value }
    }
}


#[cfg(test)]
mod tests {
    // Uses super to access the add function from the parent module.
    // See I remembered that. I am learning!
    use super::*;

    //#test tells the "Test runner" (no idea what that is yet) that this is a test function
    #[test]
    fn it_works() {
        // Adds 2 and 2. If the result equals 4, the test passes.
        // If it doesn't then it fails. 
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
// This test is designed to return a Result type, which can be used to indicate success or failure.
// Hooray for the return of Result<T,E>!
    fn it_also_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // This test is designed to fail.
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    // Tests the canhold method of the rectangle struct.
    // If the rect can hold the smaller rectangle, the test passes!
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    // And now the opposite. This should fail.
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    // Tests the add_two function to ensure it adds 2 to the input correctly.
    // It's like the game It Takes Two, but instead it adds two.
    // If anyone is reading these comments, please know that I am enjoying Rust.
    // But some of these tasks are a bit tedious at times. 
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    // Test the greeting function to ensure it contains the name Carol.
    // If the greeting does not contain the name, the test will fail and give a custom message.
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    // Tests the Guess struct to ensure it panics if the value is less than 1.
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
