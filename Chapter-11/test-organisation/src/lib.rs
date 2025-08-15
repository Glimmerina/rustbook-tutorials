// A function to add two unsigned integers together.
// Called Left and Right. We're doing this again, apparently.
// If anyone is reading this, I'm vegan. Just thought you should know.

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Calls a function called internal-adder.
// I wonder if it's related to Blackadder.

pub fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Same as last time. It returns 4 so it passes.
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    // Same as the test above, but it calls a different function.   
        #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}