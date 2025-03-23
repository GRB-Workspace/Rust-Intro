// call int function
fn call_int_function(i: i32) {
    println!("Value of i: {}", i);
}

// call string function
fn call_string_function(s: String) {
    println!("Value of s: {}", s);
}

// function to give ownership of a string to another function
fn give_ownership() -> String {
    let some_string = String::from("Hello from give_ownership function");
    some_string
}

// function to take and give ownership of a string
fn take_and_give_ownership(some_string: String) -> String {
    some_string
}

// calculate length of a string
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// function to change borrowed value
fn change_borrowed_value(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn dangling_reference() -> &String {
    let s = String::from("hello");
    &s // This will throw an error because s will be dropped after this function
}
*/

fn not_a_dangling_reference() -> String {
    let s = String::from("hello");
    s
}

pub fn references_and_borrowing_intro() {
    // Ownership and functions
    let i = 5;
    call_int_function(i);
    println!("AFTER CALLING FUNCTION: Value of i: {}", i);
    println!("---------------------------------------------");

    let s = String::from("hello");
    call_string_function(s);
    // println!("AFTER CALLING FUNCTION: Value of s: {}", s); // This will throw an error because s has been moved to the function

    let s1 = give_ownership();
    println!("Value of s1: {}", s1);

    let s2 = String::from("Hello from main function");
    let s3 = take_and_give_ownership(s2);
    println!("Value of s3: {}", s3);
    println!("---------------------------------------------");

    let s4 = String::from("Hello from calculate_length function");
    let (s5, length) = calculate_length(s4);
    println!("Value of s5: {}, Length of s5: {}", s5, length);
    println!("---------------------------------------------");

    // mutable reference
    let mut s6 = String::from("hello");
    change_borrowed_value(&mut s6);
    println!("Value of s6: {}", s6);
    println!("---------------------------------------------");

    // multiple references
    let mut s7 = String::from("hello");
    let r1 = &mut s7;
    let r2 = &mut s7;
    // println!("{}, {}", r1, r2); // This will throw an error because you can't have multiple mutable references to a variable

    // multiple references with scope
    let mut s8 = String::from("hello");
    {
        let r1 = &mut s8;
        r1.push_str(", world!");
    }
    let r2 = &mut s8;
    r2.push_str("!!");
    println!("Value of s8: {}", s8);
    println!("---------------------------------------------");

    // mutable and immutable references
    let mut s9 = String::from("hello");
    let r1 = &s9;
    let r2 = &s9;
    // let r3 = &mut s9; // This will throw an error because you can't have a mutable reference while having an immutable reference
    println!("Value of r1: {}, Value of r2: {}", r1, r2);

    let r3 = &mut s9;
    r3.push_str("!!");
    println!("Value of s9: {}", s9);
    println!("---------------------------------------------");

    // dangling references
    // let ref_value = dangling_reference();

    let ref_value = not_a_dangling_reference();
    println!("Value of ref_value: {}", ref_value);
    println!("---------------------------------------------");
}
