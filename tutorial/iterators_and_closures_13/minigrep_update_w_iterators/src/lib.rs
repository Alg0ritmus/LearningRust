use std::env;
use std::error::Error;
use std::fs;

// make everything accasible from main.rs by using
// "pub" keyword

pub struct Config{
    pub query: String,
    pub file_path: String,
    pub case_ignore: bool,
}


impl Config{

    pub fn build (
        // args: is now generic types that implements Iterator traits 
        // where Item is String (just like std::env::Args implements Iterator 
        // traits -> https://doc.rust-lang.org/stable/std/env/struct.Args.html#impl-Iterator)
        mut args:  impl Iterator<Item=String>,
                  )->Result<Config,&'static str>{

        // skip first arg which is name of our program
        args.next(); // so we call next and do nothing w return val

        let query = match args.next(){
            Some(arg_val) => arg_val,
            None => return Err("I didn't get query string"),
        };

        let file_path = match args.next(){
            Some(file_path) => file_path,
            None => return Err("I didn't get filepath"),
        };

        // &args[1] is &str, but args[1] is actually String and we can't moves it
        // bcs. we wouldn't be able to work with args[2] and so on
        // so we need to clone them
   

        let var_ignore_case = env::var("IGNORE_CASE").is_ok();


       let result_config =  Config{
            
           // cloning values is taking down runtime performance
           // but for now it's acceptable
            query, 
            file_path,
            case_ignore: var_ignore_case,

        };
       Ok(result_config)

    }
}


// as we show in Chapter 9 (Error handeling) about Box<dyn Error>
// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#where-the--operator-can-be-used
// Box<dyn Error> is trait obj, and it basically means that we can return any Error
pub fn run(config: Config)->Result<(),Box<dyn Error>>{
    
    // fs::read_to_string->Result<String>
    let contents = fs::read_to_string(config.file_path)?;
   
    // check if we have config.case_ignore on/off 

    let result = if config.case_ignore{
        search_case_insensitive(&config.query,&contents)
    }
    else{
        search(&config.query,&contents)

    };

    // print all item in vec we return from from search()
    
    // we need to use reference bcs. 1) we specified args of search like this
    // 2) we specified signature of search fn with references bcs. we don't
    // want fn to take ownership of values
    // old: for matched_line in search(&config.query,&contents){
        
    //new:

    for matched_line in result{
        println!("{matched_line}");
    }

    Ok(())
}





pub fn search<'a>(query: &str, contents: &'a str)->Vec<&'a str>{
    
    contents.lines()
        .filter(|x| x.contains(query))
        .collect()


}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str)->Vec<&'a str>{
    contents
        .lines()//iterator
        .filter(|x| x.to_uppercase().contains(&query.to_uppercase()))
        .collect()
}









// TESTS

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three,
Duct Tape.";
            assert_eq!(vec!["safe, fast, productive."],search(query,content));

    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three,
Trust me.";
            assert_eq!(vec!["Rust:","Trust me."],search_case_insensitive(query,content));

    }
}
