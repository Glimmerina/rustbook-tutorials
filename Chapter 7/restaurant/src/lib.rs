pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

//In this tutorial we are apparently opening a restaurant.
//I am calling it Rustcargo Beef. Sorry Carmy.
mod front_of_house;

pub use crate::front_of_house::hosting;

mod customer {
    use crate::front_of_house::hosting;
    use crate::back_of_house;
    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        hosting::add_to_waitlist();

        //Customer begins by ordering Rye toast then changes their mind.
        let mut meal = back_of_house::Breakfast::summer("Rye");
        //Changed the tutorial bread from Rye to Hovis. I'm a Hovis girl
        meal.toast = String::from("Hovis");
        println!("I'd like {} toast please", meal.toast);

        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }   
}

fn deliver_order() {}

pub mod back_of_house {
        pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, //Not public, the customers don't get to choose what's in season
    }
        pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

use std::fmt::Result;
use std::io::Result as IoResult;
use std::collections::*;

fn function1() -> fmt::Result {
    // The rustbook specifies this code and it returns errors, amazing.
}

fn function2() -> io::Result<()> {
    // As above.        
}