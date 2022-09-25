///////////////////////
// Try to implement Inorder tree traversal in rust
//
// https://www.geeksforgeeks.org/tree-traversals-inorder-preorder-and-postorder/
//
//
// 1) Traverse the left subtree, i.e., call Inorder(left->subtree)
// 2) Visit the root.
// 3) Traverse the right subtree, i.e., call Inorder(right->subtree)


// binary tree
use std::rc::Rc;
use std::cell::RefCell;
use crate::Next_node::{Next,Null};
#[derive(Debug)]
enum Next_node{
    Next(Rc<RefCell<Node>>),
    Null,

}


#[derive(Debug)]
struct Node{
    value: i32,
    left_child: Next_node,
    right_child: Next_node,

}

impl Node {

    fn new(val: i32)->Node{
        Node {
            value: val,
            left_child: Null,
            right_child: Null,
        }
    }

    fn insert(&mut self,N: Node){
    
        if self.value < N.value{
            //add to right
            match &mut self.right_child {
                Null => {
                    self.right_child = Next(Rc::new(RefCell::new(N)));
                    //println!("right={:?} left={:?}", self.right_child,self.left_child); 
                },
                Next(a) => a.borrow_mut().insert(N),
            }
        }
        else{
            // add to left
            match &mut self.left_child {
                Null => {
                    self.left_child = Next(Rc::new(RefCell::new(N)));
                    //println!("left={:?} left={:?}", self.left_child,self.left_child); 
                },
                Next(a) => a.borrow_mut().insert(N),
            }
        }
    }

    fn inodrer_traverse(&self){
    
        // traverse all the way to left subtree

        match &self.left_child {
            Null => (), 
            Next(a) => {
                a.borrow().inodrer_traverse()
            }
        }

        // print number
        println!("{}",self.value);

        // traverse all the way to right subtree
        match &self.right_child{
            Null => (),
            Next(a) => {
                a.borrow().inodrer_traverse()
            }
        }


    }
  
    fn preodrer_traverse(&self){
   
        // print number
        println!("{}",self.value);

        // traverse all the way to left subtree
        match &self.left_child {
            Null => (), 
            Next(a) => {
                a.borrow().preodrer_traverse()
            }
        }


        // traverse all the way to right subtree
        match &self.right_child{
            Null => (),
            Next(a) => {
                a.borrow().preodrer_traverse()
            }
        }


    }

    fn postodrer_traverse(&self){
    
        // traverse all the way to left subtree
        match &self.left_child {
            Null => (), 
            Next(a) => {
                a.borrow().postodrer_traverse()
            }
        }



        // traverse all the way to rigth subtree
        match &self.right_child{
            Null => (),
            Next(a) => {
                a.borrow().postodrer_traverse()
            }
        }
        // print number
        println!("{}",self.value);


    }
    
}

fn main() {

    let numbers = [15,50,10,22,35,70,4,12,18,24,31,44,66,90];
    // ceate root node (needs to be mutable)
    let mut root = Node::new(25);
    for i in 0..numbers.len(){
        let x = Node::new(numbers[i]);
        root.insert(x);
    }

    println!("inorder:");
    root.inodrer_traverse();
    println!("preorder:");
    root.preodrer_traverse();
    println!("postorder:");
    root.postodrer_traverse();

    //dbg!("from main a={:?}",root);

    /*
    let mut a = Node::new(1);
    let b = Node::new(2);
    let c = Node::new(3);
    let d = Node::new(-1);
    a.insert(b); // b is moved
    a.insert(c); // c is moved
    a.insert(d); // d is moved

    a.inodrer_traverse();
    */
    //dbg!("from main a={:?}",a);

}
