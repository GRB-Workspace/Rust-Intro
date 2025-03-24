pub fn example_struct_intro() {
    without_struct();
    println!("---------------------------------");
    with_struct();
    println!("---------------------------------");
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn with_struct() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_struct(&rect1)
    );
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn without_struct() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
