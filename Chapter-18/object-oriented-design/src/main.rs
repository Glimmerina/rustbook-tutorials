// Note this does not work because we do not have a Blog crate.
use blog::Post;

fn main() {
    // Allows us to make a new post
    let mut post = Post::new();

    // Adds text to it
    post.add_text("I ate a salad for lunch today");

    // Sends it for review
    let post = post.request_review();

    // Sends it to approval
    let post = post.approve();

    // Asserts that the content is what we expect it to be.
    assert_eq!("I ate a salad for lunch today", post.content());
}