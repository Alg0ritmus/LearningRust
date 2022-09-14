// bring add_one package to scope
// make sure you add "add_one" package to our
// dependecies in add/adder/Cargo.toml
use add_one;

// note that, even we've included add_one package,
// which includes rand itslef, we cant use it here
// unless we includ rand package to local Cargo.toml

// use rand;

// don't forget to include add_two crate in
// Cargo.toml dependecies
use add_two;

fn main() {
    let number_form_ad_one = add_one::add_one_function(10);
    println!("Hello, world!, 10 plus 1 is {number_form_ad_one}");

    // note that function add_random_function is using rand crate,
    // but it's not included in add/adder/Cargo.toml
    // rather it is included in add/add_one/Cargo.toml 
    let number_form_add_rand= add_one::add_random_function(10);
    println!("Hello, world!, 10 plus rand is {number_form_add_rand}");

    let number_form_add_two= add_two::add_two_function(10);
    println!("Hello, world!, 10 plus two is {number_form_add_two}");
}
