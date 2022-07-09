// https://doc.rust-lang.org/book/ch04-03-slices.html
#[warn(unused_variables)]
fn main() {
    
    // RUST OWNERSHIP SCOPE
    {
        let s = "hello"; // 's' is valid here
    } // 's' is NOT valid here
 

    {
        // String type (that type is suitable for string,
        // which value is unknown at compile time)

        let s = "hello";
        let mut s = String::from(s); // this is stored on heap (not stack)
        s.push_str(", world"); // pushin to the heap
        println!("my modyfieds String: {}",s);
    } // out of scope

    // double free error prevention by RUST
    {
        let s1 = String::from("my strin");
        let s2 = s1; // after this assignment (copy pointer, capacity and length)
        //of s1 String(bc. this is on heap not stack)  RUST automat. free s1 
        // these steps are called move (s1 moves to s2)

        //println!("this is s1 {}",s1); // gives us an error
        println!("this is s2 {}",s2);

    }

    // if we want to copy data (heap data not ust pointer that is on stack)
    // we can use .clone method
    
    {
         let s1 = String::from("my strin");
         let s2 = s1.clone();
         println!("s1 = {} \n s2 = {}",s1,s2);
    }
    
    // ownership in functions
        
    let s1 = String::from("mystring"); // bring s1 into scope
    takes_ownership(s1); // s1 "moves" into the function
    // s1 is not valid here

    let x1 = 5; // bring x1 into scope
    makes_copy(x1); // x1 is COPY (copy trait) and parsed into function
    // here it is valid to use x1
    println!("{}",x1);

    // returning values is also transfering ownership
    //
    let some_string_1 = gives_ownership(); // some_string_1 in scope
    takes_ownership(some_string_1); // some_string_1 out of scope

    let some_string_2 = String::from("alabama"); // some_string_2 in scope
    let some_string_3 = take_and_give_ownership(some_string_2);
    // some_string_2 out of scope, some_string_1 in scope
    println!("{}",some_string_3); 

    // REFERENCES and BORROWING
    // Reference for passing value to function, but dont need to return it
    // and it still be valid (passing pointer to function)
    // reference -> &
    // dereference -> *

    let calc_this_length = String::from("kasjndsdbaf");
    let length = calc_len_of_string(&calc_this_length);
    println!("length of {} is {}",calc_this_length,length); // we passed to dunction just pointer so we didnt move value
    // everything is valid here

    // BOROWING

    let mut pozdrav =  String::from("aloha");
    change(&mut pozdrav);
    println!("{}",pozdrav);
    println!("{}",pozdrav);

    // U can borrow immutable value multiple times but
    // mutable value just once

    {
        let s = String::from("aloha");

        let s1 = &s;
        let s2 = &s;
        println!("s1 = {} | s2 = {} ", s1,s2);
        println!("s1 = {} | s2 = {} ", s1,s2);
        
        // mutable pointer can be borrowed/used just once

        let mut x = String::from("aloha");

        let x1 = &mut x;
        println!("x1 = {}", x1);
        let x2 = &mut x;
        println!("x2 = {} ", x2);


    }

    // mutable and immutable references (pointers)  
    {
        let mut s = String::from("hello");

        let r1 = &s; //no problem
        let r2 = &s; // no problem
        let r3 = &mut s; // BIG PROBLEM

        // how to fix ?!
        let r1 = &s; //no problem
        let r2 = &s; // no problem
        println!("r1 = {} | r2= {}",r1,r2);
        println!("r1 = {} | r2= {}",r1,r2);
        //varibale r1 and r2 is not being used after this
        let r3 = &mut s; // BIG PROBLEM
        println!("r3 = {}",r3);
        println!("r3 = {}",r3);
    }

    
}

// functions are implemented here
//

fn takes_ownership(my_string: String) { // my_string goes into scope
    println!("{}",my_string); 
} // my_string goes out of scope here, memory is automatically freed by rust


fn makes_copy(my_num: i32) { // my_num goes into scope
    println!("{}",my_num);
} // my _num goes out of scope, nothing happens




fn gives_ownership() -> String {
    
    let s1 = String::from("ownership is given");
    s1
}

fn take_and_give_ownership(mut my_string: String) -> String {
    my_string.push_str("!!!");
    my_string
}


fn calc_len_of_string(my_string: &String) -> usize { // takes pointer not value
    let len = my_string.len();
    len
}

fn change(my_sting: &mut String) {
    my_sting.push_str("pridal som toto!");
}


fn dangle_ref() -> &String{
    let s = String::from("lalal");
    &s
}
