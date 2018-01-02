pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.state.as_mut().unwrap().add_text(&mut self.content, text);
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

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str { "" }
    fn add_text(&mut self, _content: &mut String, _text: &str) { }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> { Box::new(PendingReview {}) }
    fn approve(self: Box<Self>) -> Box<State> { self }
    fn add_text(&mut self, content: &mut String, text: &str) {
        content.push_str(text);
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> { self }
    fn approve(self: Box<Self>) -> Box<State> { Box::new(Published {}) }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> { self }
    fn approve(self: Box<Self>) -> Box<State> { self }
    fn content<'a>(&self, post: &'a Post) -> &'a str { &post.content }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn review_state() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }

    #[test]
    fn add_text_only_to_draft() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        post.add_text(" and it was good");
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
