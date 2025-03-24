enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Rarity),
}

#[derive(Debug)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

// match with Option<T>
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Option::Some(i) => Option::Some(i + 1),
        Option::None => Option::None,
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(rarity) => {
            println!("Quarter with rarity: {:?}", rarity);
            25
        }
    }
}

// Catch-all pattern
fn catch_all_pattern(some_number: u8) {
    match some_number {
        1 => println!("One"),
        3 => println!("Three"),
        5 => println!("Five"),
        7 => println!("Seven"),
        _ => (),
    }
}

pub fn match_statement_intro() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;
    println!("Penny: {}", value_in_cents(penny));
    println!("Nickel: {}", value_in_cents(nickel));
    println!("Dime: {}", value_in_cents(dime));
    println!("Quarter: {}", value_in_cents(quarter(Rarity::Epic)));
    println!("--------------------------------------------------");

    let five = Option::Some(5);
    let six = plus_one(five);
    println!("{:?}", six);
    println!("-------------------------------------------------");

    let none = Option::None;
    let none_plus_one = plus_one(none);
    println!("{:?}", none_plus_one);
    println!("-------------------------------------------------");

    catch_all_pattern(1);
    println!("-------------------------------------------------");
}
