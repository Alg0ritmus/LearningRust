fn main() {
    // iterators are lazy
    
    let v1 = vec![1,2,3];
    
    // this code by itself does basically nothing 
    let mut v1_iter = v1.iter();

    // we dont need to define iterator as mutable in for loop
    // bcs. this is done under the hood
    // note that for loop takes ownership
    for item in v1_iter.clone(){
        println!("Got: {}",item);
    }


    // using next on iterator changes internal state of Iterator
    // so we need to define v1_iter (our iterator) as mutable
    // next() works basically like popping out of vec and 
    // return 1st value
    //
    // also note that return value from next() is immutable ref.
    assert_eq!(v1_iter.next(),Some(&1));
    // you can see that 1st element gets popped out
    //dbg!(v1_iter.clone());
    assert_eq!(v1_iter.next(),Some(&2));
    assert_eq!(v1_iter.next(),Some(&3));
    assert_eq!(v1_iter.next(),None);

/*
 *    Method        |   Produces
 *    ----------------------
 *    iter()        |   immutable ref. to item
 *    into_iter()   |   owned value of item
 *    iter_mut()    |   mutable ref. to item
 * 
 * */

// Methods that call next are called consuming adaptors
// one of those methods is sum(), which "eats up "
// see src/lib.rs


// Iterator adaptors don't consume Iterator
// but rather produce different inerator
// by changing some aspect of original iterator
// e.g. methond map

    let v1 = vec![1,2,3];

    // iterators are lazy and this practically do nothing 
    // unless we consume iterator for example with collect()
    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
    println!("v1: {:?}",v1);
    println!("v2: {:?}",v2);

    


// end of main   
}
