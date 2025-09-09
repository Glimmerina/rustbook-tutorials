// Uses the trpl crate.
// trpl is not a real crate. It does not exist on crates.io.
// It provides a simple way to make HTTP requests and parse HTML. I think.
// I've had a long day and I'm tired so this one may misunderstand the Rustbook a bit.
use trpl::{Either, Html};

fn main() {
    // Get the command line arguments.
    let args: Vec<String> = std::env::args().collect();

    // trpl, when ran, will start an async runtime
    trpl::run(async {
        // Get the page titles of two URLs concurrently.
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        // Wait for the first one to complete
        let (url, maybe_title) =
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        // Prints the result to declare which URL returned first and its title.
        println!("{url} returned first");
        // Print the title if it was found.
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}

// The async function that fetches the page title of a URL.
async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}
