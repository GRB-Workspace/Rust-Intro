mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// using 'super' keyword to access the parent module
fn deliver_order() {}

// paths for structs
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// paths for enums
mod back_of_house_enum {
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn paths_intro() {
    // Absolute path
    crate::paths::front_of_house::hosting::add_to_waitlist(); // crate keyword is used to refer to the root of the current crate
    // Relative path
    front_of_house::hosting::add_to_waitlist(); // Relative path is used to refer to the parent module

    fn fix_incorrect_order() {
        cook_order();
        super::paths::deliver_order() // super keyword is used to refer to the parent module
    }

    fn cook_order() {}

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries"); // this will throw an error because seasonal_fruit is private

    let order1 = back_of_house_enum::Appetizer::Soup;
    let order2 = back_of_house_enum::Appetizer::Salad;
    println!("{:?}, {:?}", order1, order2);
}
