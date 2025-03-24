#[derive(Debug)] // This is used to print the struct with debug print
struct User {
    username: String,
    email: String,
    is_active: bool,
    age: u8,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs
#[derive(Debug)]
struct UnitLikeStruct;

pub fn structs_intro() {
    let user1 = User {
        username: String::from("Gayanuka Bulegoda"),
        email: String::from("grbulegoda@gmail.com"),
        is_active: true,
        age: 23,
    };
    println!(
        "Name: {} \nEmail: {} \nActive: {} \nAge: {}",
        user1.username, user1.email, user1.is_active, user1.age
    );
    println!("---------------------------------------------");
    println!("{:?}", user1);
    println!("---------------------------------------------");

    // Mutable structs
    let mut user2 = User {
        username: String::from("John Doe"),
        email: String::from("john@gmail.com"),
        is_active: false,
        age: 45,
    };
    println!(
        "Name: {} \nEmail: {} \nActive: {} \nAge: {}",
        user2.username, user2.email, user2.is_active, user2.age
    );
    println!("---------------------------------------------");

    // Built struct from external function
    let user3 = build_user(String::from("Jane Doe"), String::from("doe@gail.com"));
    println!("{:?}", user3);
    println!("---------------------------------------------");

    // Struct instance from other struct instance
    let user4 = User {
        username: String::from("Jane Smith"),
        email: user1.email,
        is_active: user1.is_active,
        age: user1.age,
    };
    println!("{:?}", user4);
    println!("---------------------------------------------");

    // Tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {}, {}, {}", black.0, black.1, black.2);
    println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);
    println!("---------------------------------------------");

    // Unit-like structs
    let unit_like_struct = UnitLikeStruct;
    println!("{:?}", unit_like_struct);
    println!("---------------------------------------------");
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        is_active: true,
        age: 34,
    }
}
