use std::fmt::Display;
use std::fmt::Debug;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

#[derive(Debug)]
pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Trait Bounds
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{0}

#[derive(Debug)]
pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y:T) -> Self {
        Self{
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }

    }
}
