pub struct Post {
    // Box<dyn State> is a trate object which is anything that implements State
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // state is private so the ONLY way to create a Post is to set the initial state to Draft
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        ""
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
