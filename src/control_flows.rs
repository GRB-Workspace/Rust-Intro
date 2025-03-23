pub fn control_flows_intro() {
    // if - else
    let number = 3;
    if number < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }
    println!("----------------------------------");

    // Assigning a value to a variable based on a condition
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
    println!("----------------------------------");

    // Nested if
    let number = 6;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else {
        if number % 3 == 0 {
            println!("Number is divisible by 3");
        } else if number % 2 == 0 {
            println!("Number is divisible by 2");
        } else {
            println!("Number is not divisible by 4, 3, or 2");
        }
    }
    println!("----------------------------------");

    // && and || operators
    let a = 5;
    let b = 6;
    let c = 7;

    if a < b && b < c {
        println!("a is less than b and b is less than c");
    } else {
        println!("a is not less than b and b is not less than c");
    }

    if a < b || b < c {
        println!("a is less than b or b is less than c");
    } else {
        println!("a is not less than b or b is not less than c");
    }
    println!("----------------------------------");

    // match statement
    let number = 3;
    match number {
        1 => println!("Number is 1"),
        2 => println!("Number is 2"),
        3 => println!("Number is 3"),
        4 => println!("Number is 4"),
        _ => println!("Number is not 1, 2, 3, or 4"),
    }
    println!("----------------------------------");

    // loop
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10 {
            break;
        }
    }

    // returning a value from a loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);
    println!("----------------------------------");

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // for loop (for construct)
    let array = [10, 20, 30, 40, 50];
    for element in array.iter() {
        println!("The value is: {}", element);
    }
    println!("----------------------------------");

    // for loop with range (1..4 means 1 to 3)
    for number in (1..4).rev() {
        // here rev() is used to reverse the range
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    println!("----------------------------------");

    // FizzBuzz problem (A common interview question)
    // Print numbers from 1 to 100
    // For multiples of 3, print "Fizz" instead of the number
    // For multiples of 5, print "Buzz" instead of the number
    // For multiples of 3 and 5, print "FizzBuzz" instead of the number
    for number in 1..=100 {
        if number % 3 == 0 && number % 5 == 0 {
            println!("FizzBuzz");
        } else if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", number);
        }
    }
}
