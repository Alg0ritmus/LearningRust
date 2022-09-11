use std::process;
use std::env;
use minigrep::Config; 

fn main() {
    // before
/*
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error msg: {err}");
        process::exit(1);
    });
*/



    // after

    // main difference is that we are moving(parsing ownership)
    // env::args() to Config::build()
    // we are not parsing reference like before
    let config: Config = Config::build(env::args())
        .unwrap_or_else(|err| {

            eprintln!("Error msg: {err}");
            process::exit(1);
    });



    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error {e}");
        process::exit(1);
    }

    // end of main
}

