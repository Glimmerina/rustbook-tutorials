// We can pass functions to functions. Oh god. This is getting out of hand.

fn add_one(x: i32) -> i32 {
    x + 1
}

// This function takes another function as an argument. I'm scared.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    // Passing a function as an argument to another function.
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");

    // And now we can use closures too. What a time to be alive.
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
    list_of_numbers.iter().map(ToString::to_string).collect();

    enum Status {
        Value(u32),
        Stop,
    }

    // Using closures to create a list of enum variants.
    // Closures are so powerful. I'm scared. What have we done.
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
   
   // Using the returned closures from the functions above. Merp.
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}

// Functions that, when called, return closures. 
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}

// I wish I had a witty comment to end this file with but frankly I'm just baffled.
// Is there anything Rust can't do?