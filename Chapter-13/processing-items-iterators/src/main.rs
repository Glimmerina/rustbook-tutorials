pub trait Iterator {
    // The Iterator trait defines a series of methods that allow iteration over a sequence of items.
    // The next method returns the next item in the sequence, consuming the iterator.
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

fn main() {
    
    let v1: Vec<i32> = vec![1, 2, 3];

    // Map creates a new iterator. Collect consumes it.
    // This is a code ouroboros.
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
