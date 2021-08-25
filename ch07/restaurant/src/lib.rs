mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    // Structs can have individual public or private fields.
    // Any private field requires an associated function, like summer() below,
    // to construct an instance of the struct.
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

    // A public Enum automatically has all public variants.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// Absolute "use" path
use crate::front_of_house::hosting;
// Relative "use" path
use self::front_of_house::serving;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::seat_at_table();
    // With "use" statement
    hosting::add_to_waitlist();


    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // private fields can't be accessed.
    // meal.seasonal_fruit = String::from("blueberries");
    
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
