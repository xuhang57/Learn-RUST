// A trait tells the Rust complier about functionality a particular type has and can share with other types
// We can use traits to define shared behavior in an abstract way
// We can use traits bounds to specify that a generic type can be any type that has certain behavior

// Traits are similar to a feature often called "Interface" in other languages, although with some differences

// Defining a Trait
pub trait Summary {
    fn summarize(&self) -> String;
}

// Implementing a Trait on a Type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!{"{}, by {}, ({})", self.headline, self.author, self.location}
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

let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());


// impl Summary for NewsArticle {} --> use default implementation

// Traits as Parameters
pub fn notify(item: &impl Summary) {
    println("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
// The impl Trait works for straightforward cases but it is actually syntax sugar for a longer form:
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

pub fn notify<T:Summary>(item1: &T, item2: &T) {}

// Specifying Multiple Traits Bounds with the * Syntax
pub fn notify(item: &(implement Summary + Display)) {}

pub fb notify<T: Summary + Display>(item: &T) {}

// Clearer Trait Bounds with where Clauses
fn some_function<T: Display + clone, U: CLone + Debug>(t: &T, u: &U) -> i32 {}

// we can use a where clause like
fn some_function<T, U>(t:& T, &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}

// Returning Types that Implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Lucas"),
        content: String::from("Xu"),
        reply: false,
        retweet: false,
    }
}

// Using Trait Bounds to conditionally Implement Methods
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<t> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior.
// The compiler can then use the trait bound information to check that all the concrete types used with our code provide the correct behavior
