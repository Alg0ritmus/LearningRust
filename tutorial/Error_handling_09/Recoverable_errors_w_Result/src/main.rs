use std::fs::File;
use std::io::ErrorKind;
fn main() {

    // There're 2 types of errors in Rust-lang.
    //
    // 1) Recoverable -> those types that u want to handle by urself (reading non extisting file )

    // 2) Unrecovarable -> those that can cause crash of program (go out of range in for loop etc.)


    /////////////////////////////////////////////
    // Recoverable errors w Result 
    /////////////////////////////////////////////


    // Result is enum type that is implemented like this:
    //
    // enum Result<T, E> {
    //     Ok(T),
    //      Err(E),
    // }

    // Where T is generic (if success) Type and E is Error Type (if fail)




    ///////////////////////////////////////////////////
    // Multiple ways to catch errors when reading file
    ///////////////////////////////////////////////////


    // Opening file can fail so we need to handle that
    let greeting_file_result = File::open("hello.txt");
    // error when file does not exist 
    //println!{"{:?}",greeting_file_result}; 

   
    // HOW TO HANDLE Err ?

    // 1) using match expression

/*
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result{
        Ok(file) => file,
        Err(error) => panic!("Problem w opening the file {:?}",error) 
    };
        
*/

    // 2) HANDLE MULTIPLE ERR W MATCH
    
/*
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind(){
            // if not found, print error + create file 
            ErrorKind::NotFound => {
                panic!{"File not found, really {:?}",error}
                match File::create("hello.txt") {
                    Ok(file) => file,
                    Err(e) =>panic!{"File can't be created {:?}",e}
                }
                
            },
            // if other error, print it
            other_err => panic!{"Problem w opening the file {}",other_err}
        }
    };
*/

    // 3) HANDLE ERR W MULTIPLE ERR w Closure

/*    
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!{"File couldn't be createdi {:?}",error};
                })
            }
            else{
                panic!{"Problem w opening file {:?}",error};
            }
            
    });
*/

    // 4) Unwrap vs Expect
/*    
    // if u use unwrap() u get default error msg from panic! call
    let greeting_file_result = File::open("hello.txt").unwrap();

    // using expect let you define your own err panic! msg
    let greeting_file_result = File::open("hello.txt")
        .expect("File can't be open");
*/

    let file_content = read_username_from_file_shorter();
    println!{"File content {:?}",file_content.unwrap()};
    
    
    
    
    
    
    

// end main   
}




//////////////////////////////////////////////////
// Propagating error from fn -> return Errors
//////////////////////////////////////////////////

use std::io::{self,Read};

fn read_username_from_file() -> Result<String,io::Error> {
    let username_file_result = File::open("hello.txt");

    // check if file opening is valid
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // read the file and save content to String
    match username_file.read_to_string(&mut username){
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }

}


// MAKE IT SHRTER

fn read_username_from_file_shorter()-> Result<String,io::Error>{
    // if ? expression fails, it returns immediately io::Error
    let mut username_file_result = File::open("hello.txt")?;
    
    let mut username: String = String::new();

    username_file_result.read_to_string(&mut username)?;
    // here we are returning Ok(userbame) not just username
    // bcs. Return type is Result, not String !!
    Ok(username)
        
}



// JUST w few lines of code?


fn read_username_from_file_even_shorteri()-> Result<String, io::Error>{

    let mut username: String = String::new();

        
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}



use std::fs;
fn read_username_from_file_one_line() -> Result<String, io::Error>{
    // std::fs implements fs::read_to_string method that
    // opens file and create new string, read
    // contet of file to that string and returns it, if err, return err
    fs::read_to_string("hello.txt")
    
}
