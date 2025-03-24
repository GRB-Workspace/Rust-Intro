#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// enums and structs can be used together
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// enums with data in it (like a struct)
#[derive(Debug)]
enum Message {
    Quit, // unit-like struct
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enums can have methods
impl Message {
    fn call(&self) {
        println!("Message: {:?}", self);
    }
}

// Option enum
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

pub fn enums_intro() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Four: {:?}", four);
    println!("Six: {:?}", six);
    println!("--------------------------------------");

    route(four);
    route(six);
    println!("--------------------------------------");

    let loopback = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("Home: {:?}", loopback);
    println!("--------------------------------------");

    let m = Message::Write(String::from("Hello, World!"));
    let q = Message::Quit;
    let mv = Message::Move { x: 10, y: 20 };
    let cc = Message::ChangeColor(255, 255, 255);
    println!(
        "Message: {:?}, Quit: {:?}, Move: {:?}, ChangeColor: {:?}",
        m, q, mv, cc
    );
    println!("--------------------------------------");

    m.call();
    q.call();
    mv.call();
    cc.call();
    println!("--------------------------------------");

    let some_number = Option::Some(5);
    let some_string = Option::Some("a string");
    let absent_number: Option<i32> = Option::None;
    println!(
        "Some Number: {:?}, Some String: {:?}, Absent Number: {:?}",
        some_number, some_string, absent_number
    );
    println!("--------------------------------------");

    /*
        let x: i8 = 5;
        let y: Option<i8> = Some(4);
        let sum = x + y.unwrap(); // unwrap() returns the value inside Some variant
        println!("Sum: {:?}", sum);
    */
}

fn route(ip_type: IpAddrKind) {
    println!("Route: {:?}", ip_type);
}
