use std::collections::HashMap;

pub fn hash_maps_intro() {
    // Creating a new empty HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
    println!("{:?}", scores);
    println!("-----------------------------------------");

    // Assigning values in a HashMap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // copied() method is used to copy the value from the reference
    // unwrap_or() method is used to return the value if it is Some or return the default value if it is None
    println!("Score of Blue team: {}", score);

    let no_team_name = String::from("Red");
    let score = scores.get(&no_team_name).copied().unwrap_or(0);
    println!("Score of Red team: {}", score);
    println!("-----------------------------------------");

    // Iterating over a HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    println!("-----------------------------------------");

    // HashMaps and Ownership
    let mut map = HashMap::new();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    map.insert(&field_name, field_value);
    println!("{:?}", map);
    println!("field_name: {}", field_name);
    println!("-----------------------------------------");

    // Updating a HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    println!("-----------------------------------------");

    // Only inserting a value if the key has no value
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
    println!("-----------------------------------------");

    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    println!("-----------------------------------------");
}
