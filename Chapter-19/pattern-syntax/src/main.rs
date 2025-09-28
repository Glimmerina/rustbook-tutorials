// I'M BACK! ONE WEEK LATER AND IM BACK IN MY IDE!
// OH IT FEELS GOOD. MAY THE CRAB WELCOME ME BACK INTO ITS CLAWED EMBRACE!


// Struct for a Point with an X, Y and Z coordinate.
struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

// Enum for Color with RGB and HSV variants.
// I don't like HSV but eh, guess some users would want it.
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

// Enum for a message with different variants.
// I've commented out some variants as the tutorial keeps changing the code and it was causing errors.
enum Message {
    //Quit,
    Hello { id: i32 },
    //Move { x: i32, y: i32 },
    //Write(String),
    //ChangeColor(Color),
}


fn main() {
    // Defines X and Y coordinates.
    // X = some value. 
    let x = Some(5);
    let y = 10;

    // Matches the value of X against different patterns.
    // If X is Some(50), it prints "Got 50".
    // If X is Some(n) and n equals Y, it prints "Matched, n"
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    // Prints the values of X and Y at the end.
    println!("at the end: x = {x:?}, y = {y}");

    // Demonstrates the use of the @ operator in pattern matching.
    // @ is used to bind a value to a variable while also testing it against a pattern.
    let msg = Message::Hello { id: 5 };

    // Matches the message against different patterns.
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}

// Foo is no longer called but it was earlier in the tutorial so I've kept it here.

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}