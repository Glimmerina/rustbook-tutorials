    #[test]
    fn iterator_demonstration() {
        // Creates a vector with 3 elements
        // Using the next method, it consumes the iterator one element at a time.
        // The iterator is consumed, so it cannot be used again after this point.
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

        #[test]
    fn iterator_sum() {
        // The Sum method consumes the iterator. It cannot be used after this point.
        // Like a diamond when casting resurrect in D&D, it's consumed forever.
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// This function is a little hectic.
// It uses filter with a closure to filter out shoes that don't match the given size.
// It uses the shoe size variable, and then iterates over the shoe struct instances.
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}