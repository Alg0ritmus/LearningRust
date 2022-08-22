mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

    }
}

mod back_of_house {

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
}

fn deliver_order(){}

pub fn eat_at_restaurant() {
    // Abslolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relate path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please ", meal.toast);

    // private field -> seasonal_fruit
    // meal.seasonal_fruit = String::from("bluberries");

    // choose from enums

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}

// crate shortcut to path with use keyword



mod test {
    pub mod backyard{
        pub fn dubpster(){}
    }
}



use crate::front_of_house::hosting;
use crate::test::backyard;


mod customer{
    // use crate::front_of_house::hosting;
    // use crate::test::backyard;

    pub fn drink_at_restaurant() {
        // if u declare use keyword in scope,
        // there is no need to use super keyword

       // hosting::add_to_waitlist();
       // backyard::dubpster();

        // if u declare use keyword outside of scope,
        // u need to use super keyword
        super::hosting::add_to_waitlist();
        super::backyard::dubpster();


    }
}



// import 2 types of same Name and use as keyword (just like in python)

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}


fn function2() -> IoResult<()> {
   Ok(()) 
}


// use external packages -> rand = "0.8.3"
// which is added as dependecy in cargo.toml

use rand::Rng;

fn rand_numb(){
    let secret_number = rand::thread_rng().gen_range(1..=100);
} 



// ended at
// https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html
