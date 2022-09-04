// Traits are like methods, but they're implemented like so:

// define trait
pub trait Summary {
    // define method
    // fn summerize(&self)->String;

    // define required method

    fn summerize_author(&self)->String;

    // define default summerize_author
    

    // define default method
    fn summerize(&self)->String{
        // call also required method summerize_author
        format!{"Reading from {} ..",self.summerize_author()}
    }
}

pub struct Tweet {
    pub author: String,
    pub body: String,
    pub reply: bool,
    pub retweet: bool,
}

pub struct NewsArticle{
    pub author: String,
    pub body: String,
    pub location: String,
    pub headline: String,
}

pub struct Earticle{
    pub author: String,
    pub body: String,
    pub location: String,
    pub headline: String,
}

// implement Summary trait for NewsArticle
impl Summary for NewsArticle{
    fn summerize(&self)->String{
        format!{"{} by {}, location:{}",self.headline,self.author,self.location}      }
    fn summerize_author(&self)->String{
        format!{"Author: {}",self.author}
    } 
}
// implement Summary trait for Tweet
impl Summary for Tweet{
    fn summerize(&self)->String{
        format!{"{} wrote: {}",self.author,self.body}
    }

    fn summerize_author(&self)->String{
        format!{"@{}",self.author}
    }
}

// default impl of Summary trait for Earticle
impl Summary for Earticle{

    fn summerize_author(&self)->String{
        format!{"Author unknown"}
    } 
}

// define function on trait, so it takes only types
// that are implemented on Summary trait

pub fn notify(item: &impl Summary){
    println!{"notice! {}",item.summerize()};
}

// Longer form of impl Trait is down here:

// pub fn notify<T: Summary>(item: &T){
//     println!{"notice! {}",item.summerize()};
// }


/////////////////////////////////////
// Syntax to use when multiple params/Traits
/////////////////////////////////////


// Using multiple Traits for 1 item would look like this:

// pub fn notify(item: &(impl Summary + Display)) {...}

// or this

// pub fn notify<T: Summary + Display>(item: &T) {...}
//


// if we have multiple params with multiple Traits we can us Where syntax

// instead of:
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

// we can write:

// fn some_function<T, U>(t: &T, u: &U) -> i32
// where T: Display + Clone,
//       U: Clone + Debug
// {







/////////////////////////////////////
//  Return Trait
/////////////////////////////////////

// note that u don't need to specify Summary Type 
// (like Tweet, Earticle, NewsArticle..)
// however, your function can return just 1 type
// so u cannot create smt like: if true -> Tweet, else -> Earticle
pub fn return_summarizable_tweet()-> impl Summary{
    
    Tweet {
        author: String::from("Mlon Eusk"),
        body: String::from("in case u don't need to lose a boner fast"),
        reply: false,
        retweet: false,

    }
}



/////////////////////////////////////
// Define conditional Traits on generics
/////////////////////////////////////

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// simple Trait
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// impl only if Pair<T> -> T type implements Display and PartialOrd traits
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

