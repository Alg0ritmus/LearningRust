#[derive(Debug)]
enum IpAddrKind { // kind of data type
        IPv4,
        IPv6,
}

#[derive(Debug)]
enum IpAddrKind2 {
    // u can assign different types for IPv4 and IPv6

    IPv4(u8,u8,u8,u8), // define type, and also now u can assaign value
    IPv6(String), 
    // IpAddrKind2::IPv6 is now function,
    // that takes String and returns IpAddrKind2 instance
    // and IpAddrKind2:IPv4 is now function,
    // that takes four u8 values and returns IpAddrKind2 instance
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Coin {
            Penny,
            Nickle,
            Dime,
            Quarter,
            Zelcoin(IpAddrKind),
}



fn main() {
   // use enu, if u have strictly defined options   

    // using enum
    //
    let addr1 = IpAddrKind::IPv4; // using :: bcs, namespaced
    let addr2 = IpAddrKind::IPv6;
    
    route(&addr1);
    route(&addr2);

    // 1 way to assosiate values with enums
    // here we using enum: IpAddrKind2
    {
        let addr1 = IpAddr {
            kind: addr1,
            address: String::from("127.0.0.1"),
        };

        let addr2 = IpAddr {
            kind: addr2,
            address: String::from("::1"),
        };
    
        println!("This is addr1: {:?}",addr1);
        println!("This is addr2: {:?}",addr2);

    }

    // better way for assosiating data w enums
    // here we using enum: IpAddrKind2

    {
        let addr1 = IpAddrKind2::IPv4(127,0,0,1);
        let addr2 = IpAddrKind2::IPv6(String::from("::1"));

        println!("This is addr1: {:?}",addr1);
        println!("This is addr2: {:?}",addr2);

    } 

    // Enums and functions

    {
        #[derive(Debug)]    
        enum Message {
            Quit,
            Move{x: i32, y: i32},
            Write(String),
            ChangeColor(i32,i32,i32),
        
        }

        impl Message {
            fn call(&self){
                println!("Message : {:?}", &self);

            }
        }
        println!("-------------------------------");
        let m = Message::Write(String::from("asbc"));
        m.call();

    }
    // Option <T> -> included in prelude so u dont have to import that or use ::
    // instead of nulls, implemented like This:

    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let my_num: i8 = 5;
    let absent_num: Option<i8> = Some(5);
    
    // this is an error, i8 + Option<i8> is not implemented
    // let sum = my_num + absent_num;
    
    // we have to extract value from Option<T>, using build in methods
    // https://doc.rust-lang.org/std/option/enum.Option.html

    let sum = my_num + absent_num.unwrap_or(0);
    println!("summary = {}",sum);



    // MATCH PATTERNS
    //

    {
        
        
        // Coin enum is declared at the begginin

        let coin = Coin::Penny;
        let coin = value_in_cents(coin);
        println!("value of coin is {}",coin);

        
        let coin = Coin::Zelcoin(IpAddrKind::IPv4);
        let coin = value_in_cents(coin);
        println!("value of coin is {}",coin);


    }

    // Matching witch Option<T>
    // Option T is in prelude so we dont need to
    // import it nor use ::
    
    let none = None;
    let five = Some(5);

    let six = add_one(five);
    let none = add_one(none);
    println!("this is after adding to five {:?}",six);
    println!("this is after adding to none {:?}",none);

    // Catch-all with match
    // syntax x => ()
    //

    let my_number: u8 = 11;

    match my_number {
        // n is vaue tat match return 
        n if n >10 => println!("You Lucky!, You got {}",n),  
        n if n > 5 => (), // do nothing 
        // catch all here
        _ => println!("Never mind u can try once again"),
    }
    println!("Ownership of my_number {}", my_number);
    
}

fn value_in_cents(mycoin: Coin) -> u8 {
    match mycoin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::Zelcoin(ip_type) => {
            println!("ZelCoin spotted for IP add type: {:?}",ip_type);
            100
        },
    } 
}


fn add_one(x: Option<i32>) -> Option<i32> {

    match x {
    
        Some(i) => Some(i+1),
        None => None,
    }
}

fn route(ip_kind: &IpAddrKind) {
    println!("MY ip kind: {:#?}", ip_kind);
}
