fn main() {

    // Create a mutable HashMap to store scores.
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing a value in the HashMap using a reference to a String.
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Loop that runs through the HashMap and prints each key-value pair. Team names are the keys, scores are the values.
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Defines two strings to be used as keys and values in a HashMap.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    // Creates a mutable HashMap and inserts the field name and value into it.
    // The strings become invalid after this point, as they are moved into the HashMap.
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // Values in a HashMap can be updated by inserting a new value with the same key.
    // The previous value associated with the key will be replaced.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    // Using the `entry` method to insert a value if the key does not already exist.
    // If the key exists, it will not change the value.
    // At the time I write this, I do not know why you would need this, but good to know it exists!
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    // Example of counting occurrences of words in a string using a HashMap.
    // The keys are the words, and the values are the counts of how many times each word appears.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}