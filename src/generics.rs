// Generic functions
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generic Structs
struct Point<T> {
    x: T,
    y: T,
}

// Multiple Generic Types
struct MultiplePoint<T, U> {
    x: T,
    y: U,
}

// Generic in methods
impl<T, U> MultiplePoint<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

// Distance from origin
impl MultiplePoint<f32, f32> {
    fn distance_from_origin(&self) -> f64 {
        ((self.x.powi(2) + self.y.powi(2)) as f64).sqrt()
    }
}

// Mixer method
impl MultiplePoint<i32, f32> {
    fn mixup<V, W>(self, other: MultiplePoint<V, W>) -> MultiplePoint<i32, W> {
        MultiplePoint {
            x: self.x,
            y: other.y,
        }
    }
}

// Generics for Enums
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub fn generics_intro() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    println!("--------------------------------------------------");

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let char = Point { x: 'a', y: 'b' };

    println!("Integer Point: x = {}, y = {}", integer.x, integer.y);
    println!("Float Point: x = {}, y = {}", float.x, float.y);
    println!("Char Point: x = {}, y = {}", char.x, char.y);

    println!("--------------------------------------------------");

    let integer_and_float = MultiplePoint { x: 5, y: 4.0 };
    let char_and_float = MultiplePoint { x: 'a', y: 4.0 };

    println!(
        "Integer and Float Point: x = {}, y = {}",
        integer_and_float.x, integer_and_float.y
    );
    println!(
        "Char and Float Point: x = {}, y = {}",
        char_and_float.x, char_and_float.y
    );

    println!("--------------------------------------------------");

    // read the x value of the integer_and_float point
    println!(
        "x value of integer_and_float point: {}",
        integer_and_float.x()
    );
    // read the y value of the integer_and_float point
    println!(
        "y value of integer_and_float point: {}",
        integer_and_float.y()
    );

    println!("--------------------------------------------------");

    // Distance from origin
    let point = MultiplePoint { x: 3.0, y: 4.0 };
    println!("Distance from origin: {}", point.distance_from_origin());

    println!("--------------------------------------------------");

    // Mixer method
    let p1 = MultiplePoint { x: 5, y: 10.4 };
    let p2 = MultiplePoint { x: 3, y: 10.2 };
    let p3 = p1.mixup(p2);
    println!("Mixer method: x = {}, y = {}", p3.x, p3.y);

    println!("--------------------------------------------------");

    // Generics for Enums
    let some_number = Option::Some(5);
    let no_number: Option<i32> = Option::None;

    println!("Some number: {:?}", some_number);
    println!("No number: {:?}", no_number);

    println!("--------------------------------------------------");

    let success: Result<i32, &str> = Result::Ok(5);
    let failure: Result<i32, &str> = Result::Err("Failed to execute the code");

    match success {
        Result::Ok(value) => println!("Success: {}", value),
        Result::Err(error) => println!("Error: {}", error),
    }

    match failure {
        Result::Ok(value) => println!("Success: {}", value),
        Result::Err(error) => println!("Error: {}", error),
    }

    println!("--------------------------------------------------");
}
