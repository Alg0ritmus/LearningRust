// ReffCell<T> is smart pointer that allows us to has multiple references to 1 obj.
// ReffCell<T> allows immutable or mutable borrows checked at runtime 
// ReffCell<T> allows us to mutate value inside the ReffCell<T>
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List{
    // here we wrap i32 into RefCell (so we can mutate i32)
    // even when multiple owners owns particular list
    Cons(Rc<RefCell<i32>>,Rc<List>),
    Nil,
}


fn main() {

    let x =5;
    // let y = &mut x; 
    // this wouldn't compile

    /////////////////////////////////////////////
    // Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
    /////////////////////////////////////////////

    // Recall: Rc<T> allows u to have multiple owners for immutable data
    // using Rc<T>, that honds RefCell<T> allows us to have multiple owner of
    // the same data, that can be mutated


    // here we redefine ConsList, in which we can mutate value

    // define value accroding to List definition
    let value = Rc::new(RefCell::new(5));

    // fill List with reference to value
    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));

    // crate list b that has reference to list a
    let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));


    println!("a before = {:?}", a);

    // mutate value of list a

    //note: *value -> RefCell<>

    // *value.borrow_mut() -> mutable borrow of inner value,
    // we can acctually mutate
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
