pub fn collections_intro() {
    // Ways to create a new vector
    // 1. Using the Vec::new() function
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);
    println!("----------------------------------------------");

    // 2. Using the vec! macro to create a vector with initial values
    let v = vec![1, 2, 3];
    println!("{:?}", v);
    println!("----------------------------------------------");

    // Updating a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    println!("{:?}", v);
    println!("----------------------------------------------");

    // pop() method removes the last element from the vector and returns it
    let mut v = vec![1, 2, 3];
    let three = v.pop();
    println!("{:?}", three);
    println!("----------------------------------------------");

    // Accessing elements in a vector
    let v = vec!['a', 'b', 'c', 'd', 'e'];
    let third: &char = &v[2];
    println!("The third element is {}", third);
    println!("----------------------------------------------");

    // Using the get method to access elements in a vector
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    let fifth: Option<&char> = v.get(4);
    println!("{:?}", fifth);
    println!("----------------------------------------------");

    // Iterating over the values in a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    println!("----------------------------------------------");

    // Iterating over mutable references in a vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // *i is used to dereference i and get the value it refers to
    }
    println!("{:?}", v);
    println!("----------------------------------------------");

    // Using an enum to store multiple types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Int(6),
        SpreadsheetCell::Int(9),
    ];
    println!("{:?}", row);
    println!("----------------------------------------------");

    // Dropping a vector drops all its elements
    let v = vec![1, 2, 3];
    println!("{:?}", v);
    // v goes out of scope and is dropped here
    // println!("{:?}", v); // This will throw an error
}
