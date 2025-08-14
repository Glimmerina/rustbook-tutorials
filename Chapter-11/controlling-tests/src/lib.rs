// Creates an i32 variable called "a" but always returns 10 anyway.
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

// Public function that adds 2 to a u64 number. Cannot be negative because it's unsigned.
pub fn add_two(a: u64) -> u64 {
    a + 2
}


#[cfg(test)]
mod tests {
    // Uses super so it can access the function in the parent module
    use super::*;

    #[test]
    // Lies about the test name to make it sound like it will pass
    // it always returns 10 so this test will pass.
        fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }
    // will fail because it expects 5 but the function always returns 10
    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }

    // Tests for the add_two function
    // Equals a plus 2. a= the number passed to the function by the test.
    // All tests will pass.
        #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }

    // This test will pass because it adds two plus two and expects 4.
    // Had to change this from add to add_two because it didn't like add by itself.
    #[test]
    fn it_works() {
        let result =add_two(2);
        assert_eq!(result, 4);
    }

    // So the Rustbook didn't give code for this. 
    // It's only purpose is to test the `cargo test -- --ignored` command.
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}