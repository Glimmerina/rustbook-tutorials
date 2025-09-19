fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // This is a long series of "if" statements for setting colour.
    // It's entirely to demonstrate that massive chains of if/else statements are less efficient than Match statements.

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // Similarly, a While Let loop is more efficient than a loop with a break condition.
    // Both are forms of patterns.

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    // A For loop is also a form of pattern matching. The value that follows the For part is a pattern.
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    // So apparently a variable is also a pattern.
    // This means that function parameters are patterns too.
    // Am I also a pattern? How deep does this rabbit hole go?!
    let (x, y, z) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);
}

// Function parameters are also pattern. Oh god.    
fn foo(x: i32) {
    
}

// Destructuring in function parameters is also possible.
// Literally everything is a pattern. Except the things that aren't. I'm scared.
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}
