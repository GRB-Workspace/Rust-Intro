pub fn lifetimes_intro() {
    // Preventing Dangling References with Lifetimes
    let x: i32 = 5;
    let y: &i32;
    {
        let z: i32 = 10;
        y = &z;
        println!("y: {}", y);
    }
    // println!("y: {}", y); // Error: z does not live long enough

    // Generic Lifetimes in Functions
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);

    println!("--------------------------------------------------");

    // Thinking in Terms of Lifetimes
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is: {}", result);
    } // string2 goes out of scope here, but string1 is still in scope

    // Lifetime Annotations in Struct Definitions
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Lifetime Elision
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("The first word is: {}", word);

    println!("--------------------------------------------------");

    // Three rules for Lifetimes
    // 1. Each parameter that is a reference gets its own lifetime parameter.
    // 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
    // 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.

    // Lifetime Annotations in Method Definitions
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);

    println!("--------------------------------------------------");

    // 'static Lifetime
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);

    println!("--------------------------------------------------");

    // Generic type parameters, trait bounds, and lifetimes all in one function!
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest_with_an_announcement(string1.as_str(), string2, "The longest string is");
    println!("The longest string is: {}", result);

    println!("--------------------------------------------------");
}

// Generic Lifetimes in Functions (With Lifetime annotation syntax)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Lifetime Annotations in Struct Definitions
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime Elision
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// Lifetime Annotations in Method Definitions
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// Generic type parameters, trait bounds, and lifetimes all in one function!
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}
