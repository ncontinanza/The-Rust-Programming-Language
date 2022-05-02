use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;
    // The implementation of this method will be given by each type that implements the trait.
    // Default implementations are also supported.
    fn summarize2(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarize_author(&self) -> String;
}

// Implementing a trait on a type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }


    // No need to make any reference to summarize2 here.
    // However, if the only method in the trait was a default one, we should declare an empty impl such as:
    // impl Summary for NewsArticle {}
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

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Traits as Parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// For some more complex cases, trait bound syntax may be helpful:
pub fn notify2<T: Summary>(item1: &T, item2: &T) {}

// We can also specify more than one trait bound:
pub fn notify3(item: &(impl Summary + Display)) {}

pub fn notify4<T: Summary + Display>(item: &T) {}

// If it gets too hard to read, we can use a where clause:
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{ return 1; }

// Returning types that implement Traits
fn returns_summarizable() -> impl Summary { // This function returns some type that implements Summary
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// Conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> { // This is always implemented
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> { // This is only implemented for types that implement PartialOrd and Display.
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


