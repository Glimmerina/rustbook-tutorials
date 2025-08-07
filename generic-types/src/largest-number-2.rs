fn largest(list: &[i32]) -> &i32 {
    // The find largest tool is now a function rather than being in main.
    let mut largest = &list[0];

    // Iterate through the list to find the largest number. 
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    // Same number list as before.
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    // Finds the largest number in the list and prints it.
    println!("The largest number is {result}");

    // Now creates a new number list whilst keeping the original Largest variable.
    // It can now find the largest number between both lists without copying code.
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {result}");
}