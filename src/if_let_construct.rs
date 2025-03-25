enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Rarity),
}

#[derive(Debug)]
enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

pub fn if_let_construct() {
    // Using match statement
    let some_value: Option<i32> = Some(5);
    match some_value {
        Some(value) => println!("Value: {}", value),
        _ => println!("No value"),
    }
    println!("---------------------------------------");

    // Using if let construct
    if let Some(value) = some_value {
        println!("Value: {}", value);
    } else {
        println!("No value");
    }
    println!("---------------------------------------");

    // Using if let construct with enum
    let coin = Coin::Quarter(Rarity::Epic);
    if let Coin::Quarter(rarity) = coin {
        println!("Quarter coin with rarity: {:?}", rarity);
    } else {
        println!("Not a quarter coin");
    }
    println!("---------------------------------------");
}
