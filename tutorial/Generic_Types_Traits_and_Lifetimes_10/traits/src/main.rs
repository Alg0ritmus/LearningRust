// create "shortcut"
use traits::{Summary, Tweet, Earticle,notify,return_summarizable_tweet};
// bring to scope
pub mod lib;
fn main() {
    //////////////////////////////////////////
    // Traits: Defining Shared Behavior 
    //////////////////////////////////////////

  
    // Traits are like methods, but they're implemented like so:
    // Proper impl is in library crate src/lib.rs, so we'll use it here !
/*
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

    // implement Summary trait for NewsArticle
    impl Summary for NewsArticle{
        fn summerize(&self)->String{
            format!{"{} by {}, location:{}",self.headline,self.author,self.location};
        }
    }
    // implement Summary trait for Tweet
    impl Summary for Tweet{
        fn summerize(&self)->String{
            format!{"{} wrote {}",self.author,self.location};
        }
    }
*/


    let tweet =  Tweet {
        author: String::from("Elon Musk"),
        body: String::from("in case u need to lose a boner fast"),
        reply: false,
        retweet: false,

    };

    println!{"New tweet: {}",tweet.summerize()};
    println!{"New tweet from: {}",tweet.summerize_author()};


    let e_article = Earticle{
        author: "Albert".to_string(),
        body: "Theory of relativity...".to_string(),
        location: "Berlin".to_string(),
        headline: "Quick doodle".to_string(),

    };

    // Earticle uses default method impl in Summary trait
    println!{"New Earticle: {}",e_article.summerize()};

    // call function, that is implemented on Summary trait

    notify(&e_article);

    let tweet2 = return_summarizable_tweet();
    println!{"New tweet: {}",tweet2.summerize()};
    println!{"New tweet from: {}",tweet2.summerize_author()};
}     
