use std::process;
use std::env;
use minigrep::Config; 
// and make shortcut for Config struct from lib.rs
// so u dont have to type minigrap::Config, but just Config

fn main() {
    // CLI I/O program should takes 2 arg, filepath and string to srearch
    // $ cargo run -- searchstring example-filename.txt
    // using "--" indicates that args is for our program, rather than for cargo


    // takes input args
    let args: Vec<String> = env::args().collect();
    // args() returns iter w args, collect() just convert iter to vec
    // args() -> takes only valid unicode values, return collection of String
    // args_os() -> takes (in)valid unicode vals, returns collection of OsString
    // OsString is spicific to OS, so u need to take some actions to make
    // it functional, for simplicity we'll be using args() then
    
    // dbg!(args); // debug macro to print 


    // using match

/*    
    let config: Config = match Config::build(&args){
        Ok(c) => c,
        Err(e) => {
            println!{"{e}"};
            process::exit(1); // terminate the process
        }
    };

*/  

    // unwrap_or_else method + closure:
    // unwrap_or_else (is defined on Result<T, E>) returns T if success or E on fail
    // and we can then use closure to return error
    // + closure is anonymous function and in this case I've defined it to take
    // "err" as argument, this inner value "err" is filled up with result of unwrap_or_else
    // which is in this case E (Error msg)
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        // eprintln! macro prints error msg to standard error output
        // so if we type "cargo run > output.txt" and there is an error
        // it will print error msg just on screen not to output.txt
        // to output.txt will be changed only if program runs successfully
        eprintln!("Error msg: {err}");
        process::exit(1);
    });



   // check for errors w if let syntax 
   // there's no need to use unwrap_or_else bcs. we are only interested in
   // err value
    if let Err(e) = minigrep::run(config) {
        // eprintln! macro prints error msg to standard error output
        eprintln!("Application Error {e}");
        process::exit(1);
    }

    // end of main
}

