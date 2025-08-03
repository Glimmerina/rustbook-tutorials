fn main() {

    //Create a mutable String called s.
    let mut s = String::new();

    //Create a new string with an initialised string value.
    let data = "initial contents";

    // Convert the string slice to a String.
    // This is done by calling the `to_string` method on the string slice.
    let d = data.to_string();

    // The method also works on a literal directly:
    let d = "initial contents".to_string();

    //Or we can create a String from a string literal using the `String::from` function.
    let literal = String::from("initial contents");

    //Using the push method we can append to a String, but only if it's mutable.
    let mut GrowString = String::from("foo");
    GrowString.push_str("bar");

    //Sometimes we want to keep the original string. S2 remains unchanged after use.
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    //Push, like a vector, will append to the string.
    let mut s = String::from("lo");
    s.push('l');

    //We can also concatenate two strings using the `+` operator.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    //Conconate multiple strings using the `format!` macro.
    // This does not take ownership of the strings, so they can still be used afterwards.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
}