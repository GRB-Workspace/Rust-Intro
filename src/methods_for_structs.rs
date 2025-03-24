struct Rectangle {
    width: u32,
    height: u32,
}

// impl block is used to define methods for a struct (implementation block)
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // methods with the same name of a field
    fn width(&self) -> bool {
        self.width > 0
    }

    // methods as getters
    fn get_width(&self) -> u32 {
        self.width
    }

    // methods with multiple parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
            || self.width > other.height && self.height > other.width
    }

    // associated functions
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    // Multiple impl blocks are allowed
    fn another_method(&self) -> u32 {
        self.width * self.height
    }
}

pub fn methods_for_structs_intro() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("-------------------------------------------");

    if rect1.width() {
        println!("The width of the rectangle is {}", rect1.width());
    } else {
        println!("ERROR! The width of the rectangle is 0");
    }
    println!("-------------------------------------------");

    println!("The width of the rectangle is {}", rect1.get_width());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let can_hold = rect1.can_hold(&rect2);
    println!("Can rect1 hold rect2? {}", can_hold);
    println!("-------------------------------------------");

    let square = Rectangle::square(10);
    println!("The area of the square is {} square pixels.", square.area());
    println!("-------------------------------------------");

    // Multiple impl blocks are allowed
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.another_method()
    );
    println!("-------------------------------------------");
}
