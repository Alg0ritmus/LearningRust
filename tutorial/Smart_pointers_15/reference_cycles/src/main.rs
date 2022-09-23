use std::cell::RefCell;
use std::rc::Rc;
use crate::List::{Cons,Nil}; // make shortcut so we dont need to use List::Const... just Const

#[derive(Debug)]
enum List{
    // crate mutable ref(RefCell) to Rc<List>
    // which allows us have multiple owners + we can mutate <Rc<List>> node
    Cons(i32,RefCell<Rc<List>>), // i32 is not wrapped in RefCell bcs. we are not gonna change vals
    Nil,

}

impl List{
    
    fn trail(&self)->Option<&RefCell<Rc<List>>>{
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    } 
}

fn main() {

    // unsafe rust cant prevent from mem leaks
    // so be carefull there is a chance u can make some 

    ////////////
    // Creating a Reference Cycle


    // structure: b --> a --> nil
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
                    
    println!("b next is: {:?}",b.trail());
   
    // every Rc::strong_count holds at least 1 ref (itself)
    println!("b ref counter {:?}",Rc::strong_count(&b));
    println!("a ref counter {:?}",Rc::strong_count(&a)); 

    if let Some(link) = a.trail(){
        // link -> Rc<List> ...
        *link.borrow_mut() = Rc::clone(&a);
    }
    
    // structure now: b <--> a      ||       b --> a --> b

    println!("b ref counter after mutation {:?}",Rc::strong_count(&b));
    println!("a ref counter after mutation {:?}",Rc::strong_count(&a)); 


    // now we will crate memory leak (stack overflow)
    // which result in runtime error:
    // thread 'main' has overflowed its stack
    // fatal runtime error: stack overflow

    // println!("b pointing to: {:?}",b.trail());
    // println!("a pointing to itself : {:?}",a.trail());

    // when we create ref cycle like this,
    // then a and b whas strong_count set to 2
    // after b is dropped, and strong_count of Rc<List> allocated on heap 
    // will be set to 1 not to 0 bcs. There is sill reference to b (by a)
    // and Rc<List> (allocated on heap) wont be dropped bcs. it still have reference to it
    //
    // when a is dropped, the same rule applies,
    // Rc<List> of a decrease to 1, but Rc<List> instance wont be dropped
    // bcs. it still has reference
    // Therefor there will be mem leaks
    //
    // note that a, and b are just pointers to some instance allocated on heap
    // so really when compiler is about to drop vlaues a and b at the end of
    // the main, it drops those references but wont clean up memory bcs. there
    // are still some references to those vals



    //////////////////////////////////////////////////////
    // Prevent from reference cycle/ Rc<T> into Weak<T>

    // Rc::clone increases Strong_count, and instance of Rc<T> is cleaned
    // only if strong_count = 0;
    //
    // strong_reference is how you can shere ownership
    // but you can also make a weak_reference, that dont share ownership
    // calling Rc::downgrade create Weak<T> (weak reference)
    // that holds weak_count of type Rc<T>,
    // weak_count doesnt need to be 0 to be cleaned
    


    // to demonstrate weak reference we create Tree Structure
    //
   

    // we want to share ownerships of children nodes (so we use Rc<Node>) so we can 
    // access children values directly.
    // And also we want to be able to modify which children our node is referencing to
    // (we want to be able to update Vec) so we use RefCell<>
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>

    }


    // Our tree:
    //        branch
    //         |  
    //       [leaf]


    let leaf = Rc::new(Node{
        value: 5,
        children: RefCell::new(vec![])
    });

    let branch = Rc::new(Node{
        value: 10,
        children: RefCell::new(vec![Rc::clone(&leaf)])
    });

    // we shared ownership using Rc::clone, so leaf has now 2 owners (leaf and branch)
    // we can traverse from branche to leaf like this: branch.children, but
    // we can't go other way around so it is imposible to go from leaf to branch, bcs. leaf
    // doesnt know that there is any relationship between leaf and branch


    // here we will define Weak reference (so we prevent cycle ref.) to reference parent node
    // bcs. we wanna be able to traverse from leaf to branch node

    // we'll create parent value in our Node stuct, just to create weak reference to parent
    //
    // Note: if parent gets dropped (at the end of main), all weak_ref. can be (and will be)
    // dropped also

    
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct Node_2 {
        value: i32,
        children: RefCell<Vec<Rc<Node_2>>>,
        parent: RefCell<Weak<Node_2>>,
    }



    let leaf = Rc::new(Node_2{
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),

    });
   
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
        );

    { 

    let branch = Rc::new(Node_2{
        value: 15,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),

    });
    
    // reference children to parent
    //
    // downgrade() to create weak_ref
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
        );
    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
        );
    }
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
        );
    
    // upgrade() just prints out references
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());



    // upgrade(upgrade())  --returns--> Option<Rc<T>>
   // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
   // now we can access parent node, note that when we print leaf.parent
   // we dont get stack overflow, rather it prints (Weak)










    // Our tree:
    //          A
    //      /   |   \
    //    [B,   C,   D]
}

























