# Summary of chapter 14


## Specify particular package
we can specify particular package to be tested like so:  

`cargo test -p <name_of_package>`

## Creating the docs
We can create docs using `cargo docs` and then open it in browser  
using `cargo docs --open`.

Now we can add som description to our docs using special comments:

1) ///  -> support Markdown notation, suitable for functions/ structs description  

commonly used  section:  
/// #Example  
/// #Panics  
/// #Errors  
/// #Safety  

2) //!  ->  comments that contains items  



## Pub use, for re-exporting (most of the time in lib.rs)
Re-export modules in lib.rs like this:  

```
pub use self::kinds::PrimaryColor; 
pub use self::kinds::SecondaryColor; 
pub use self::utils::mix; 
```
so we can than use this modules in main.rs like this:

```
use <packagename>::PrimaryColor;
use <packagename>::SecondaryColor;
use <packagename>::mix;
```


## Upload to crate.io

1) create account on crates.io (sign up w github)
2) get access token
3) log in using: `cargo login <access_token>` command
4) fill up Cargo.toml w mandatory information (description,   
license, license-file, documentation, homepage or repository)
5) push your package to crates.io: `cargo publish`

