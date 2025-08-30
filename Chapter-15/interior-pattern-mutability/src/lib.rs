// Public trait Messenger.
// It has a method send that takes an immutable reference to self and a string slice.
// This trait is used to define the behavior of sending messages.
pub trait Messenger {
    fn send(&self, msg: &str);
}

// Public struct LimitTracker that holds a reference to a Messenger, a value, and a maximum limit
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

// Implementation of LimitTracker
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    // Method to set the current value and send messages based on usage thresholds
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        // Check the percentage of max and send appropriate messages. Cannot exceed 1.
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }

    // Test to ensure that a warning message is sent when usage exceeds 75%
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--

        // Does not work as mock_messenger is not brought into scope.
        // I do not know why the Rustbook does it this way. 
        // The book claims this code is supposed to panic. But only I panic. Why.
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}