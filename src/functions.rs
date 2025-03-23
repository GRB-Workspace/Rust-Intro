fn another_function(num: i32, text: &str, character: char) {
    println!(
        "The number, text, and character are: {}, {}, {}",
        num, text, character
    );
}

fn expression_example() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}

// Return values from functions
fn sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn sum_with_difference(num1: i32, num2: i32) -> (i32, i32) {
    let sum = num1 + num2;
    let difference = num1 - num2;
    (sum, difference)
}

// Early return
fn early_return(num: i32) -> i32 {
    if num < 5 {
        return 0;
    }
    num
}

pub fn functions_intro() {
    another_function(44, "Hello, World!", 'A');
    println!("---------------------------------------------");

    expression_example();
    println!("---------------------------------------------");

    let result = sum(5, 10);
    println!("The sum of 5 and 10 is: {}", result);
    println!("---------------------------------------------");

    let sum_with_difference = sum_with_difference(10, 5);
    println!("The sum and difference is {:?}", sum_with_difference);
    println!("---------------------------------------------");

    let early_return = early_return(4);
    println!("The early return value is: {}", early_return);
    println!("---------------------------------------------");
}
