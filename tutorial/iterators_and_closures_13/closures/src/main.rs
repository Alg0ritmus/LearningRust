use std::thread;
use std::time::Duration;
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor{
    Red,
    Blue,
}

struct Inventory{
    shirts: Vec<ShirtColor>,
}

impl Inventory{
    fn giveaway(&self, user_preference: Option<ShirtColor>)->ShirtColor{
        user_preference.unwrap_or_else(|| self.get_most())
    }

    fn get_most(&self)->ShirtColor{
        let mut red_conut = 0;
        let mut blue_count = 0;

        for item in &self.shirts{
            match item{
                ShirtColor::Red => red_conut+=1,
                ShirtColor::Blue => blue_count+=1,
            }
        }

        if red_conut>=blue_count{
            ShirtColor::Red
        }
        else{
            ShirtColor::Blue
        }

    }
}

fn main() {

    let my_inv = Inventory{
        shirts : vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue]
    };

    
    let user_preference1 = Some(ShirtColor::Red);
    let giveaway_item = my_inv.giveaway(user_preference1);
    println!{"User preference was {:?}, and he got {:?}",
    user_preference1,giveaway_item};


    let user_preference2 = None;
    let giveaway_item = my_inv.giveaway(user_preference2);
    println!{"User preference was {:?}, and he got {:?}",
    user_preference2,giveaway_item};


    // define annotation type for closure
/*    
    let expensive_closure = |num: u32| -> u32{
        println!{"caluculate slowly"};
        thread::sleep(Duration::from_secs(2));
        num
    };
*/


    // function syntax vs closure syntax
    
/*
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;
*/

    // closures can use just 1 type

/* not working
    // closure that takes x and return x
    let example_closure = |x| x;

    let n = expensive_closure(String::from("hello"));
    let x = expensive_closure(5);

*/


    // closures can perform  immuable borrow, mutable borrow and
    // move (take ownership) value and it's done automaticall by
    // evaluating the action that is executed in closure body


    // Closure Immutable borrow
    //////////////////////////////
    
    let my_vec = vec![1,2,3,4];

    // by definition of our closure body 
    // we only read value so here is immutable borrow
    let only_borrows = || println!("From closure: {:?}",my_vec);

    println!{"{:?}",my_vec}; // immutable borrow
    only_borrows(); // we can immutably borrow multiple time
    println!{"{:?}",my_vec}; // immutable borrow

    
    // Closure Mutable borrow
    //////////////////////////////

    let mut my_vec = vec![1,2,3,4];
    println!("Before mutable borrow {:?}",my_vec);
    // here we accutally update my_vec so we need to use
    // mutable borrow here
    let mut my_mutable_borrow = || my_vec.push(5);

    // not valid bcs. we can immutably borrow value when it's 
    // already borrowed mutably

    // println!("{:?}",my_vec);

    my_mutable_borrow(); //here mutable borrow ends so we can than call
    // my_vec as mutable ref.
    println!("After mutable borrow {:?}",my_vec);



    // Moving in Closure
    //////////////////////////////

    // Note: there are 3 ways that define closure (3 Trait bounds)

    // 1) FnOnce -> can be called at least once
    //              all closures that moves value out of their body will
    //              use this type of Trait boud
    //
    // 2) FnMut -> can be called more than once
    //          -> can't move value out of body, but it can change vals
    //          if values are implemented as mutable borrow
    //
    // 3) Fn -> don't move value out of body
    //       -> don't mutate value
    //       -> can be called more than once, without mutating enviroment
    //       which could be used multiple times concurrently
    //
    

    // example of unwrap_or_else, which uses FnOnce() Trait Bound

/* This is implemented outside of crate so this wont complie (this is 
 * just for explanation)
 
    impl Option<T>{
        // T and F are generics
        // f is closure (similar to function) that can by anything (generic F)
        pub fn unwrap_or_else(self,f: F)->T
            where
            // here we're sayin that our closure needs to implement FnOnce 
            // Trait Bound
            F: FnOnce() -> T // FnOnce bound says: F type must be called at least
            // once, takes no args, and returns value of type T
            

            // We used FnOnce bcs. we are moving value out of body of closure
            {
                match self {
                    Some(x) => x,
                    None => f(), // when we call unwrap_or_else we need to
                    // define body of our closure carefully, so it returns value
                    // of type T
                    //
                    // for example:
                    // let a: String;
                    //
                    // Option(a).unwrap_or_else(|| String::from("This needs to 
                    // by type String"));
                }
            }

    }
*/


    // example of FnMut Trait Bound


    // sort_by_key is defined on FnMut Trait Bound, which means we do not
    // move val out of closure body, we can call closure more than once
    // and we can mutate value

    #[derive(Debug)]
    struct Rectangle{
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    
    // closure gets 1 arg and it also return 1 value to 
    // be ordered (but we cant see it here, rather
    // look at definition of sort_by_key)
    list.sort_by_key(|r| r.width);
    // Reason why is this function impl to take FnMut closure
    // is that this closure is called multiple time (on each item)
    // and we does not capture, mutate of move anything
    //
    //

    println!("{:#?}",list);
    
    // THIS WON'T WORK
/*
    let new_array = vec![]; // lets say we want to count vals amount of closer calls by addind values to vec

    list.sort_by_key(|r| {
        new_array.push(value); //pushing to another array moves value to another
        //array which is restricted in FnMut Trait Bound!!!
        // however if this closure would implement FnOnce we could then perform
        // such closure but only ONCE!
        r.width
    });
*/

    

    // BUT THIS WILL WORK

    let mut count_sort_operations = 0; 
    list.sort_by_key(|r| {
        count_sort_operations+=1;
        r.width
    });
    println!("{:#?}, sorted in {count_sort_operations} operation",list)

//end of main
}
