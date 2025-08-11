fn main() {
    // Makes a list of numbers with the largest number almost midway through.
    let number_list = vec![34, 50, 25, 100, 65];

    // Initialise a variable equal to the first element of the list.
    let mut largest = &number_list[0];

    // Loops through each number in the list, replaces the value of the 
    // "largest" variable if it's larger than the current one.
    // I'm vegan, btw
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    // Prints the largest number found in the list.
    println!("The largest number is {largest}");
}
