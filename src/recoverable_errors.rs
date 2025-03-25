use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub fn recoverable_errors_intro() {
    /*
    Recoverable Errors with Result<T, E> Enum
    2 variants of Result<T, E> Enum
        01. Ok(T)
        02. Err(E)
    */

    // 01. Ok(T)
    let result: Result<i32, &str> = Result::Ok(5);
    println!("Result: {:?}", result);

    // 02. Err(E)
    let result: Result<i32, &str> = Result::Err("Error");
    println!("Result: {:?}", result);

    println!("--------------------------------------------------");

    // File reading example
    let file = File::open("hello.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // alternative approach with unwrap_or_else
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // unwrap and expect methods
    let file = File::open("hello.txt").unwrap();
    // unwrap is a shortcut method that returns the value if the Result is Ok, and panics if the Result is Err
    let file = File::open("hello.txt").expect("Failed to open hello.txt");
    // expect is similar to unwrap, but it allows us to specify a custom panic message

    // Propagating Errors
    fn read_username_from_file() -> std::result::Result<String, io::Error> {
        let file = File::open("hello.txt");

        let mut file = match file {
            Ok(file) => file,
            Err(error) => return Err(error),
        };

        let mut s = String::new();

        match file.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(error) => Err(error),
        }
    }

    let username = read_username_from_file();
    match username {
        Ok(username) => println!("Username: {}", username),
        Err(error) => println!("Error: {}", error),
    }
    println!("--------------------------------------------------");

    // Propagating Errors with the ? Operator
    fn read_username_from_file2() -> std::result::Result<String, io::Error> {
        let mut file = File::open("hello.txt")?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(s)
    }

    let username = read_username_from_file2();
    match username {
        Ok(username) => println!("Username: {}", username),
        Err(error) => println!("Error: {}", error),
    }
    println!("--------------------------------------------------");

    // More concise version of the above code
    fn read_username_from_file3() -> std::result::Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    let username = read_username_from_file3();
    match username {
        Ok(username) => println!("Username: {}", username),
        Err(error) => println!("Error: {}", error),
    }
    println!("--------------------------------------------------");

    // Use of the ? Operator to return the error to the calling function
    fn read_username_from_file4() -> std::result::Result<String, io::Error> {
        let s = std::fs::read_to_string("hello.txt")?;
        Ok(s)
    }

    let username = read_username_from_file4();
    match username {
        Ok(username) => println!("Username: {}", username),
        Err(error) => println!("Error: {}", error),
    }
    println!("--------------------------------------------------");
}
