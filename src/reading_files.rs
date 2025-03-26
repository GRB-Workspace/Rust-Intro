use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

// Version 1
fn reading_files_v1() -> io::Result<()> {
    // open the file
    let mut file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening file: {:?}", error);
            return Err(error);
        }
    };

    // initializing a string to store the content of the file
    let mut content = String::new();

    // read the content of the file
    file.read_to_string(&mut content)?;

    // print the content of the file in the console
    println!("{}", content);

    Ok(())
}

// Version 2
fn reading_files_v2(filename: &str) -> Result<String, io::Error> {
    // open the file
    let mut file = File::open(filename)?;

    // initializing a string to store the content of the file
    let mut content = String::new();

    // read the content of the file
    file.read_to_string(&mut content)?;

    Ok(content)
}

// Version 3 - Reading Files Line by Line
fn reading_files_v3() -> io::Result<()> {
    // open the file
    let file = File::open("hello.txt")?;

    // read the content of the file line by line using BufReader
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}

pub fn reading_files_intro() {
    reading_files_v1().unwrap();

    println!("-----------------------------------");

    match reading_files_v2("hello.txt") {
        Ok(content) => println!("{}", content),
        Err(error) => eprintln!("Error reading file: {:?}", error),
    }

    println!("-----------------------------------");

    reading_files_v3().unwrap();
}
