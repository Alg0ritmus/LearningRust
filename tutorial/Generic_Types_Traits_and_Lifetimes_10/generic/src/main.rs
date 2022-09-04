fn main() {

    ///////////////////////////////////
    //  Generic Types
    ///////////////////////////////////


   ///////////////////////////////// 
   // Let's write duolicate fn, that finds largest value in vec
   /////////////////////////////////
   fn largest_i32(list: &[i32])->&i32{
        let mut largest_num = &list[0];
        for number in list{
            if largest_num < number{
                largest_num = number;
            }
        }

        largest_num
   }

   fn largest_char(list: &[char])->&char{
        let mut largest_char = &list[0];
        for character in list{
            if largest_char < character{
                largest_char = character;
            }
        }

        largest_char
   }

    let my_list = vec![0,3,10,100,-1,2,99];
    let ref_to_largest_num = largest_i32(&my_list);
    println!("largest number is {:?}",ref_to_largest_num);

    let my_list = vec!['c','a','z','f'];
    let ref_to_largest_num = largest_char(&my_list);
    println!("largest char is {:?}",ref_to_largest_num);



   /////////////////////////////////
   // make 1 fn instead of 2 using generics
   /////////////////////////////////
   
    // generic type T has no "rules" of comparison, but we can use
    // std::cmp::PartialOrd trait, which implements comparison over
    // default types like i32, char etc.. 
    fn largest<T: std::cmp::PartialOrd>(list: &[T])->&T{
        let mut largest_item = &list[0];

        for item in list{
            if largest_item < item{
                largest_item = item;
            }
        }
        largest_item
    }


    let my_list = vec![0,3,10,100,-1,2,99];
    println!{"largest number using generics {:?}",largest(&my_list)};
    let my_list = vec!['c','a','z','f'];
    println!{"largest char using generics {:?}",largest(&my_list)};




   /////////////////////////////////
   // In struct definition
   /////////////////////////////////
   
    struct Point<T>{
        x:T,
        y:T,
    }

    // Note: that 'x' and 'y' needs to be the same types
    // like this

    let p1 = Point{ x:1, y: 2};
    let p2 = Point{ x:0.1, y: 0.2};
    
    // This is invalid, bcs. x is float and y is int
    // (different types):
    // let p3: Point{ x=0.1, y= 2};
    
    // if u want to able to set different types
    // to either x and y, you need to use 2 generics
    // in strcut like this:

    struct Point_versatile<T,U>{
        x:T,
        y:U,
    }
    //since we've defined generics T and U, this is valid now
    let p3 = Point_versatile{ x:0.1, y: 2};




   /////////////////////////////////
   // In enums definition
   /////////////////////////////////

   // Example no.1 Option:

    enum Option<T>{
        Some(T),
        None,
    }

   // Example no.2 Result:
   
    enum Result<T, E>{
        Ok(T),
        Err(E),
    }



   /////////////////////////////////
   // In method definition
   /////////////////////////////////

    struct Point_w_impl<T>{
        x:T,
        y:T,
    }

    impl<T> Point_w_impl<T>{
    
       fn x(&self)->&T{
            &self.x
       } 
    }

    let p1 = Point_w_impl {x:4,y:1};
    println!{"Point.x()={:?}",p1.x()};

    // we can declare impl method just for certain types
    // e.g. f32 like this

    impl Point_w_impl<f32>{
        fn distance_from_origin(&self)->f32{
            (self.x.powi(2) + self.x.powi(2)).sqrt()
        }
    }

    let p1 = Point_w_impl {x:4.1,y:1.3};
    println!{"Point.x()={:?}",p1.distance_from_origin()};
    

    let p2 = Point_w_impl {x:4,y:1};
    // this is not valid, bcs. we didn't implement distance_from_origin
    // on i32 type
    // println!{"Point.x()={:?}",p2.distance_from_origin()};



    // Mix up 2 Points and create 3rd one
    
    struct my_point<X1,Y1>{
        x: X1,
        y: Y1,
    }

    impl<X1,Y1> my_point<X1,Y1>{
        fn mixup<X2,Y2>(self,other: my_point<X2,Y2>)->my_point<X1,Y2>{
        
            let new_point = my_point{
                x : self.x,
                y : other.y,
            };

            new_point
        }
    }

    let p_1 = my_point {x:3, y: 5.4};
    let p_2 = my_point {x:"String literal", y: 'c'};

    let mixed_up_p = p_1.mixup(p_2);

    // p_1 & p_2 will be moved !!
    // println!{"p_1 cords x={}, y={}",p_1.x,p_1.y};
    // println!{"p_2 cords x={}, y={}",p_2.x,p_2.y};

    // only p_3 is valid
    println!{"mixed_up_p cords x={}, y={}",mixed_up_p.x,mixed_up_p.y};

// end of main
}
