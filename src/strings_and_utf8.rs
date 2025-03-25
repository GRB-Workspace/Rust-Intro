pub fn strings_and_utf8_intro() {
    // Rust strings are UTF-8 encoded
    let s = "Hello, world!";
    println!("{}", s);
    println!("----------------------------------------------");

    // Creating a new empty string
    let mut s = String::new();
    println!("{}", s);
    println!("----------------------------------------------");

    // Using the to_string() method to create a new string
    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);
    println!("----------------------------------------------");

    // to_string() method can also be called on a string literal
    let s = "initial contents".to_string();
    println!("{}", s);
    println!("----------------------------------------------");

    // Using the String::from() function to create a new string
    let s = String::from("initial contents");
    println!("{}", s);
    println!("----------------------------------------------");

    // Updating a string
    let mut s = String::from("foo");
    s.push_str("bar"); // push_str() appends a string slice to a String
    println!("{}", s);
    println!("----------------------------------------------");

    // Push a single character
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);
    println!("----------------------------------------------");

    // Using the + operator to concatenate strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    println!("{}", s3);
    println!("----------------------------------------------");

    // Using the format! macro to concatenate strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    println!("----------------------------------------------");

    // Indexing into strings
    /*
    let s = String::from("hello");
    let h = s[0];
    println!("{}", h); // This will throw an error because Rust strings are UTF-8 encoded and the indexing operation is not allowed
    */

    // Slicing Strings
    let hello = "नमस्ते";
    let s = &hello[0..3];
    println!("{}", s);

    // Iterating over the characters in a string
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    println!("----------------------------------------------");

    // Iterating over the bytes in a string
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    println!("----------------------------------------------");
}
