// Box<T> is smart pointer that can be used to 3 main things:
//
// 1) When you have a type whose size can’t be known at compile time and you want to use a value of
// that type in a context that requires an exact size
//
// 2) When you have a large amount of data and you want to transfer ownership but ensure the data
// won’t be copied when you do so
//
// 3) When you want to own a value and you care only that it’s a type that implements a particular
// trait rather than being of a specific type


fn main() {
    //
    // 1) using Box<T> to store data on the heap
    ////////////////////////////////////////////

    // storing i32 value on the heap

    let b = Box::new(5);
    println!("b on heap = {}",b);

    // recursive types
    //
    // cons(construct function) list -> data structure, comes from  Lisp and it basically is linked-list made up with
    // nested pairs

    // pseudocode of cons list
    // (1, (2, (3, Nil))), where Nil is basically Null (Note that Null is absent in rust)
    

    // Let's define cons list in Rust
    
    #[derive(Debug)]
    enum List{
        Cons(i32, Box::<List>), // note: that this is recursive type (tuple) and it doesn't have known size at compile time
            // so it gives us complie error
        Nil,
    }

    use List::{Cons,Nil}; 
    let a = Cons(5, Box::new(Nil));
    let b = Cons(5, Box::new(a));

    println!("b: {:?}", b);

}

