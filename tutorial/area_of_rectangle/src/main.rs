use std::io;
#[derive(Debug)] // attribute for debug -> usful for printing struction using {:?}
struct Rectangle {
    width: u32,
    height: u32,
}

// implement method for struct Rectangle
// assosiated functions are called by ::
impl Rectangle {
    // create constructor using assosiated function
    fn square(size: u32) -> Rectangle { 
        // assosiated function (not method,
        // bc. &slef is not present) 
        Rectangle{
            width: size,
            height: size
        }
    }

    fn calc_area(&self) -> u32 {
        self.width*self.height
    }

}
// Structure can have multiple impl,
// at the end its the same as have 1 long impl
impl Rectangle {
    fn is_same(&self, another_rec: &Rectangle) -> bool {
        if self.width == another_rec.width && self.height==another_rec.height {
            true
        }
        else{
            false
        }
    }

    fn can_hold(&self, another_rec: &Rectangle) -> bool {
        self.width>another_rec.width && self.height>another_rec.height
    }
}

fn main() {

    let mut width = String::new();
    let mut height = String::new();
    println!("Zadaj width:");
    io::stdin().read_line(&mut width).expect("smths goes wrong");

    let width = match width.trim().parse() {
        Ok(num) => num,
        Err(e) => 0
    };

    println!("Zadaj height:");
    io::stdin().read_line(&mut height).expect("smths goes wrong");

    let height = match height.trim().parse() {
        Ok(num) => num,
        Err(e) => 0
    };

    let rectangle = Rectangle {
        width,
        height,
    };

    let area_of_rect = rect_area(&rectangle);
    println!("height * width = {}",area_of_rect);
    println!("rect{:?}",rectangle); // debug mode print (need to use debud derive above the struct definition)
    println!("rect{:#?}",rectangle); // pretty debug output
    dbg!(&rectangle); // another way to print struct in debud derive 
    // note: dbg! takes ownership so we pass reference
    // dbg prints error as stderr | println prints error as stdout
    
    //calc area using method
    
    println!("rect area = {}",rectangle.calc_area());

    let rec2 = Rectangle {
        width: 10,
        height: 20
    };
    let rec3 = Rectangle {
        width: 10,
        height: 20
    };
    let rec4 = Rectangle {
        width: 9,
        height: 10
    };
        
    println!("{}",rec2.is_same(&rec3)); // true
    println!("{}",rec2.is_same(&rec4)); // fale
    println!("{}",rec2.can_hold(&rec3)); //flase
    println!("{}",rec2.can_hold(&rec4)); // true
    
    // create square via assosiated function 

    let my_square =  Rectangle::square(20); // use :: with assosiated fn
    dbg!(&my_square);
}

fn rect_area(rectangle: &Rectangle) -> u32 { // using borrow here,
    //cause we dont want to own structure,
    //just borow (afrer function it wouldn't valid)
    rectangle.width*rectangle.height
}
