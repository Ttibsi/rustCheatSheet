pub struct Post {
    // state: Option<Box<dyn State>>,
    content: String
}

pub struct DraftPost {
    content: String
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        // return self.state.as_ref().unwrap().content(self);
        return &self.content;
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        return PendingReviewPost {
            content: self.content
        };
    }
}

pub struct PendingReviewPost{
    content: String
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        return Post {
            content: self.content
        };
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>; 
    fn approve(self: Box<Self>) -> Box<dyn State>; 

    // Default implementation that means it's not needed in every impl block
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        return "";
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return Box::new(PendingReview {});
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        return Box::new(Published {});
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        return &post.content;
    }
}
