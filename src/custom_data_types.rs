struct Person {
    name: String,
    age: u8,
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub fn custom_data_types_intro() {
    /*
    Custom types
    Rust allows you to define custom data types using,
      - structs and
      - enums.

    Structs
    A struct, or structure, is a custom data type that lets you name and package together multiple related values that make up a meaningful group.
    Normally structs declaration is done outside the function
    */
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("The name of the person is: {}", person.name);

    /*
    Enums
    An enum is a way to define a type by enumerating its possible variants.
     */
    let red = TrafficLight::Red;

    match red {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Get ready to go!"),
        TrafficLight::Green => println!("Go!"),
    }
}
