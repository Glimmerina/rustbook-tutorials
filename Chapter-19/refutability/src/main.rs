fn main() {
    // This is a regutable pattern. It will not compile.
        let Some(x) = some_option_value;

    // If the pattern does not match, the code will skip the code in the curly brackets.
    // This allows the code to continue executing even though we goofed it.
        let Some(x) = some_option_value else {
        return;
    };

    // Now it's irrefutable again, which will make the compiler give us a warning.
    // It works, it just doesnt make sense
        let x = 5 else {
        return;
    };
}

// Was this really the entire subchapter?