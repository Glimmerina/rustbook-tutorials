// Defines a struct that maintains a list of integers and an average of those integers.
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    // Creates a new AveragedCollection with an empty list and an average of 0.0.
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    // A function that, when called, removes the last element from the list and updates the average.
    // But only if the matched result is Some(value).
    // Also updates the average after removing an element.
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    // Returns the current average of the integers in the list.
    pub fn average(&self) -> f64 {
        self.average
    }

    // A private method that recalculates the average of the integers in the list.
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}