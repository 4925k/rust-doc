mod front_of_house;
fn deliver_order() {}

mod back_of_house {
    // enums will denote all fields public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // structs can have each field private or public
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
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// Bringing Paths into Scope with the use Keyword
use create::front_of_house::hosting;
pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::seat_at_table();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);

    // not allowed since it is private
    // meal.seasonal_fruit = String::from("Mango");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("{order1} {order2}");

    // Bringing Paths into Scope with the use Keyword
    hosting::add_to_waitlist();
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
