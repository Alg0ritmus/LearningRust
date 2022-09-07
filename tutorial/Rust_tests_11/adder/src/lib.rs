#[derive(Debug)]
struct Rectangle{
    height: u32,
    width: u32,
}

impl Rectangle{
    fn can_hold(&self, other: &Rectangle)->bool{
        self.width > other.width && self.height > other.height
    }
}

fn add_two(int: i32)->i32{
    int+2
}
#[cfg(test)]
mod tests {
    #[test] // annotation to say that this function is test
    // u can then run tests w cargo test
    fn exploration() {
        assert_eq!(2+2, 4);
    }


    // each test is running in separated thread, so if fn fails, thread dies
    // and test will be evaluated as FAILED
    #[test] // make test that fails
    fn another(){
        panic!{"This test will fail"};
    }

    use super::*; // use super::* to include struct Rectangle, bcs now it's out of scope

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle{
            width: 10,
            height: 20,
        };
        let smaller = Rectangle{
            width: 5,
            height: 10,
        };
        
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn larger_cannot_hold_smaller(){
        let larger = Rectangle{
            width: 1,
            height: 2,
        };
        let smaller = Rectangle{
            width: 5,
            height: 10,
        };
        
        assert!(!larger.can_hold(&smaller));
    }
  

    // asseet_eq && assert_ne
    #[test]
    fn it_adds_two(){
        // add custom msg to assert
       assert_eq!(add_two(2),4,"we can add custom msg"); 
    }

    // should_panic attribute, so test passes if code return Error msg (panic)
    
    fn check_number(int: i32)->i32{
        if int < 1 {
            panic!("Number '{}' is too small, need to be <1;100>",int)
        }
        if int > 100{
            panic!("Number '{}' is too big, need to be <1;100>",int)

        }
        int
    }

    #[test]
    //#[should_panic]  // if check_number ever panicked, it passes test
    #[should_panic(expected = "is too big")] 
    //if panic msg contains specific substring "is too big", it passes test
    fn greater_than_100(){
        check_number(200);
    }


    // USING Result<T, E> as result
    
    #[test]
    fn return_result()->Result<(),String>{
        if 2+2==4{
            Ok(())
        }
        else{
            Err(String::from("2+ 2 is != 4"))
        }
    }
    


}
