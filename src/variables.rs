pub fn variables_intro() {
    println!("Hello, world!");

    // Variables and Mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("---------------------------------------------");

    // Shadowing
    let y = 5;
    let y = y + 1;
    println!("The value of y is: {}", y);

    println!("---------------------------------------------");

    // Block scope
    let z = 8;
    println!("The value of z is: {}", z);
    {
        let z = 12;
        println!("The value of z is: {}", z);
    }
    let z = z + 1;
    println!("The value of z is: {}", z);

    println!("---------------------------------------------");

    // Shadowing with different types
    let a = 2;
    println!("The value of a is: {}", a);
    let a = "Hello, world!";
    println!("The value of a is: {}", a);

    println!("---------------------------------------------");

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    const MIN_POINTS: i32 = 10;
    println!("The value of MIN_POINTS is: {}", MIN_POINTS);

    println!("---------------------------------------------");

    // Thread sleep for 3 seconds
    std::thread::sleep(std::time::Duration::from_secs(3));
}
