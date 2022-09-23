// Note that ReffCell uses unsafe rust and ownership is evaluated during runtime
// not compile time, so it can cause some small downsize in performance


// create Messenger trait
pub trait Messenger {
    fn send(&self, msg: &str);
}



// create struct, that takes generics...
// in order to hold reference in struct, u need to provide lifetime
pub struct LimitTracker<'a, T: Messenger>{
    messanger: &'a T,
    value: usize,
    max: usize,
}


// implement some methods on LimitTracker, and
// we are sayin' that it is requirement for T 
// to implemets Messenger trait

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
   pub fn new(messanger: &'a T, max: usize)->LimitTracker<'a, T>{
        
       LimitTracker{
            messanger,
            value:0,
            max,
       }

   }
    
   // implementing setter for value on our LimitTracker
   pub fn set_value(&mut self, value: usize){
        
       self.value = value;

       let percentage_of_max =  self.value as f64 / self.max as f64;

       // send method is implemented on Messenger struct

       if percentage_of_max >= 1.0 {
            self.messanger.send("Error: You are over your quota!");
       }
       else if percentage_of_max >= 0.8 {
            self.messanger.send("Urgent warning: You are over 80% of your quota!");
       }
       else if percentage_of_max >= 0.75 {
            self.messanger.send("Warning: You are over 75% of your quota!");
       }


   }
}


#[cfg(test)]
mod test {

    use std::cell::RefCell;
    use super::*;


    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger{

        fn new()-> MockMessenger{

            MockMessenger{
                // sent_messages: vec![],
                
                // instead of default vec![] we can wrap vec!
                // into RefCell, so we can then mutate internal value
                // in this case vec
                sent_messages: RefCell::new(vec![]),
            }

        }
    }


    // implement Messenger trait for our new MockMessenger struct
    impl Messenger for MockMessenger{

        fn send(&self, message: &str){
            // push to sent_messages Vec
            // this won't complie bcs. &self is immutable referece
            // and we cant mutate it 

            //self.sent_messages.push(message.to_string());


            // This is perfect case for RefCell<T>, in which we 
            // can mutate interior value

            self.sent_messages.borrow_mut().push(message.to_string());
            // borrow_mut() is method impl. to get mutable borrow 
        } 

    }


    #[test]
    fn it_sends_an_over_75_percent_warning_message(){

        // we created new instance of MockMessenger
        // which stores error msg from LimitTracker
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger,100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        // also we need to get use borrow() to get immutable ref of vec
        // bcs. it is wrapped inside of RefCell
        
        // When creating immutable and mutable references, 
        // we use the & and &mut syntax,
        // respectively. With RefCell<T>, 
        // we use the borrow and borrow_mut methods


        // note that even with borrow_mut() we can use multiple
        // mutable borrows in 1 scope  (instead of compile error we get 
        // runtime error)
        // for more, check this:
        // https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#keeping-track-of-borrows-at-runtime-with-refcellt
    }

}





