// Use asparagus struct -> just for abbreviation, so u dont
// need to type whole | use crate::garden::vegetables::Asparagus just Asparagus 
use crate::garden::vegetables::Asparagus;

// include/ import code from file garden.rs, which include vegetables.ts in garden file
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm gorwinfg {:?}!",plant);
}
