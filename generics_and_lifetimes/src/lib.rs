// NOTE: the trait OR the type must be local to this crate to be able to implement
//       the trait on the type

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub trait Summary {
    // All types that have this trait must implement summarize
    fn summarize(&self) -> String; // This is abstract, but sometimes it is nice to have a default

    // This implements summarize with a default string if it is not implemented on a type
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
    // If we ONLY have defaults, we have to have an empy impl block for Summary on NewsArticle
}

// We implement the trait Summary on the type NewsArticle by defining the summarize method
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Traits become very useful when we make abstract arguments for functions
// This function works for any type that implements the Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// The above is syntax sugar for the longer form TRAIT BOUND
pub fn notify_trait_bound<T: Summary>(item1: &T, item2: &T) {
    // Two generics that must be the same type!
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}
// Notice that we declare the generic type before we specify the parameter
// The short form is useful when we just want them to both have Summary, but the
// long form is required if we want both types to be the same, but also generic

// Multiple trait bounds can be specified as well if you need something to implement
// BOTH Summary and Display. And we can use the "where" clause to help
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}
