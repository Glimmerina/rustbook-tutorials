struct Point<T, U> {
    // This is a generic struct that can hold two values of different types.
    // This is because we defined two generic type parameters, T and U.
    x: T,
    y: U,
}

fn main() {
    // Create instances of Point with different data types.
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}