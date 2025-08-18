// Search function for a text search application.

// Searches a query string in the contents of a file.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Mutable vector to store the results.
    let mut results = Vec::new();

    // Iterates through each line in the contents.
    // If the line contains the query, it adds the line to the results.
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // This test checks if the search function returns an empty vector when the query is not found in the contents.
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}