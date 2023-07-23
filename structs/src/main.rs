use std::fmt::{Debug, Display};

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// struct Color(true, true);

fn main() {
    let mut user1 = build_user("boy".to_string(), "two".to_string());

    println!("{:?}", user1.summarize());
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {}
impl Summary for User {
    fn summarize(&self) -> String {
        String::from("ehehehehe")
    }
}

fn today() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

pub fn notify(text: &impl Summary) -> String {
    text.summarize()
}

pub fn notify_bare<T: Summary + Debug>(text: &T) -> String {
    text.summarize()
}

fn do_stuff<T, U>(x: T, y: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("{}", x);
    println!("{:?}", y);
    0
}
