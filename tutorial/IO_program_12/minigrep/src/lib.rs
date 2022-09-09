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

    pub fn build (args: &[String])->Result<Config,&'static str>{
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }


        // &args[1] is &str, but args[1] is actually String and we can't moves it
        // bcs. we wouldn't be able to work with args[2] and so on
        // so we need to clone them
   

        let var_ignore_case = env::var("IGNORE_CASE").is_ok();


       let result_config =  Config{
            
           // cloning values is taking down runtime performance
           // but for now it's acceptable
            query: args[1].clone(),
            file_path: args[2].clone(),
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


// note: we need to specify which argument lifetime is 
// connected to the lifetime of the return value
pub fn mine_search<'a>(query: &str, contents: &'a str)->Vec<&'a str>{
    let mut matched_lines: Vec<&str> = vec![];

    let collection_of_lines: Vec<&str> = contents.split('\n').collect();

    // check every line
    for line in collection_of_lines.iter(){
        // check line if it contains query
        if let Some(_v) = line.find(query){
            matched_lines.push(line);
        }

    }
    matched_lines

}

pub fn mine_search_case_insensitive<'a>(query: &str, contents: &'a str)->Vec<&'a str>{
    let mut matched_lines: Vec<&str> = Vec::new();

    for line in contents.lines(){
        if line.to_uppercase().contains(&query.to_uppercase()){
            matched_lines.push(line);
        }
    }

    matched_lines

}


// note: we need to specify which argument lifetime is 
// connected to the lifetime of the return value
pub fn search<'a>(query: &str, contents: &'a str)->Vec<&'a str>{
    let mut result = Vec::new();


    // check every line
    for line in contents.lines(){
        // check line if it contains query
        if line.contains(query){
                result.push(line);
        }

    }
    result

}




pub fn search_case_insensitive<'a>(query: &str, contents: &'a str)->Vec<&'a str>{
    let mut matched_lines: Vec<&str> = Vec::new();
    let query = query.to_uppercase(); // this is better than mine impl.
    // bcs. u transform query to uppercase just once

    for line in contents.lines(){
        if line.to_uppercase().contains(&query){
            matched_lines.push(line);
        }
    }

    matched_lines

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
