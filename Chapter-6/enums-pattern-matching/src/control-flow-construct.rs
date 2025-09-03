// Enum for different types of US coins. I don't know what US coins are called.
#[derive(Debug)] // Debug for printing in console.
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    //I'm not writing every state for the sake of this tutorial.
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    value_in_cents();
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}