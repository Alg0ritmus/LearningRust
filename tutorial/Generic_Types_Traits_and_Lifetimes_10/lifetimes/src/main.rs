use std::fmt::Display;
fn main() {

    {// this works
        let x = "abcd";
        let y = "xyz"; // you can pass &str bcs. they has 'static lifetimes

        // 'static lifetime live as long as whole program live

        let result = longest(x,y);
        println!("longest string {}",result);
    }


    /*  {// this is NOT working
        let x = "abcd";
        {

        let y = "xyz";

        let result = longest(x,y);
        } // result gets out of scope here

    // result is no valid here
    println!("longest string, but bad scopes {}",result);
    }
    */



  

    /*
       {//but this is NOT working
       let string1 = String::from("long string is long");
       let result;
       {
       let string2 = String::from("xyz");
    // as we specified return valur from longest() has
    // the same lifetime as shortest lifetime of 2 string refs
    // (in this case its lifetime of string2)
    result = longest(string1.as_str(), string2.as_str());
    }// string2 drops here, and result could potentionally reference to string
    // which can cause a dangling pointer
    println!("The longest string is {}", result);
    }
    */



    //////////////////////////
    // experimental part
    //////////////////////////

    /*

       {// experiment no.1 bending rules -> https://stackoverflow.com/a/23977218
    // but still not working
    let string1 = String::from("opt A");
    let result;
    {
    let string2 = "opt B!"; // we can pass this
    let string3 = String::from("opt X"); // we can't pass this
    let string4: &str = &string3[..];  
    // we need to move a value to result
    result = longest(string1.as_str(),string4); 
    }
    println!("Experiment no.1 The longest string is {}", result);
    }
    */

    {
    let string1 = String::from("opt A");
    let result;
    {
    let string2 = "opt B!"; // we can pass this
    let string3 = String::from("opt X"); // we can't pass this
    // we need to move a value to result
    result = longest(string1.as_str(),string2); 
    }
    println!("Experiment no.1 The longest string is {}", result);
    }


    // LIFETIME ref in struct

    // in order to hold reference in struct, u need to provide lifetime
    struct RefStruct<'a>{
        part: &'a str,
    }

    let novel = String::from("asdluisaf.lsadfsda");
    let first_word = novel.split(".").next().expect("not contain .");
    let i = RefStruct{
        part: first_word,
    };
    println!("struct ref,{:?}",i.part);
    


    // using Generic Type Parameters, Trait Bounds, and Lifetimes Together
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
        );
    println!("The longest string is {}", result);
}
// function that evaluates longest string slice
// u need to specify lifetimes bcs. compiler doesn't
// know if lifetime of x and y are valid

// by assigning lifetime 'a, we say to compiler, that
// lifetime of x and y is the same, that is, shorter 
// lifetime of these two


// The longest function definition specifying that all
// the references in the signature must have
// the same lifetime 'a
//
// In other words, the generic lifetime 'a will get
// the concrete lifetime that is equal to the
// smaller of the lifetimes of x and y
fn longest<'a>(x: &'a str, y: &'a str)-> &'a str{
    if x.len() > y.len(){
        x
    }
    else{
        y
    }


}



fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
    ) -> &'a str
where
T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

