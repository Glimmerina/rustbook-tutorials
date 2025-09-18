// Oh god we're defining so many structs and implements in this tutorial aaaaaa

// Makes a struct called Post. This struct has two fields: state and content.

// Note from Future me: We had to rebuild this later in the tutorial. 
// I have commented out this version of Post and the Impl of it.


/* pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
} */

// Implements methods for Post. Gives it a constructor, a method to get content,
// a method to add text, a method to request review, and a method to approve.
/* impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
    }

    }

        pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

        pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
 */

// Defines a trait called State. This trait has three methods: request_review,
// approve, and content. The first two methods take ownership of the state and
// return a new state. The content method takes a reference to the post and
// returns a string slice.
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// Defines three structs: Draft, PendingReview, and Published. Each struct
// implements the State trait.
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// Creates the Published struct and implements the State trait for it.
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

// We create post again. Why did we do this the tutorial is not clear at all.
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

// We implement methods for Post and DraftPost again with no State trait this time.
// Older version is now commented out.
impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}


impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}