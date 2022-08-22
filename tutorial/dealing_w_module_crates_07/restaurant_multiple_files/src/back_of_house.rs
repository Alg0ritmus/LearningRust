fn fix_incorrect_order(){
    cook_order();
    super::deliver_order();
}

fn cook_order(){}
// if we use pub on struct, all fields of struct are private by default
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast{
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast{
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }

    }
}

// if we use pub on enums, all fields are public, in contrast to struct

pub enum Appetizer {
    Soup,
    Salad,
}
