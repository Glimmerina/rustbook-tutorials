fn largest_i32(list: &[i32]) -> &i32 {
    // As per before, the find largest tool is now a function rather than being in main.
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    // This function searchest for the largest character rather than the largest number.
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T>(list: &[T]) -> &T {
    // This is a generic function that can find the largest item in a list of any type.
    // It uses the same logic as the previous functions but works with any type that implements the
    // `PartialOrd` trait, which allows comparison.
    // Currently causes an error because we need to specify the trait bound.
    // However this is not covered in 10.1, apparently we will cover it later.
    
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}