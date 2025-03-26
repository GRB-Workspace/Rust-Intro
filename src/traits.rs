// Define a trait
pub trait Summary {
    fn summarize(&self) -> String; // abstract method
}

// Implement the trait for a struct
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// Implement the trait for another struct
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

// Default implementation for a trait
pub trait SummaryDefault {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl SummaryDefault for NewsArticle {}

// Multiple methods in a trait
pub trait SummaryMultiple {
    fn summarize_author(&self) -> String;

    fn summarize_data(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl SummaryMultiple for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

// Traits as parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
pub fn notify_bound<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

// Define multiple trait with the + syntax
pub fn notify_multiple(item: &(impl Summary + SummaryMultiple)) {
    println!("Breaking news! {}", item.summarize());
    println!("Breaking news! {}", item.summarize_author());
    println!("Breaking news! {}", item.summarize_data());
}

// Clearer Trait Bound with where Clauses
pub fn notify_where<T>(item: &T)
where
    T: Summary + SummaryMultiple,
{
    println!("Breaking news! {}", item.summarize());
    println!("Breaking news! {}", item.summarize_author());
    println!("Breaking news! {}", item.summarize_data());
}

// Return a type that implements a trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// Using Trait Bounds to Conditionally Implement Methods
pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Conditionally implement methods for a Generic Type
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn traits_intro() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    println!("--------------------------------------------------");

    // Article with the default implementation
    let article: NewsArticle = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());

    println!("--------------------------------------------------");

    // Multiple methods in a trait
    println!("Author: {}", article.summarize_author());
    println!("Data: {}", article.summarize_data());

    println!("--------------------------------------------------");

    // Traits as parameters
    notify(&tweet);

    println!("--------------------------------------------------");
}
