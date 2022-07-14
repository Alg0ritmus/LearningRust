struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
}

fn main() {
    // STRUCTS BASICS
    //  
    let mut user1 = User {
        username: String::from("Pato"),
        email:  String::from("skuska@gmail.com"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("user1 name is {}",user1.username);

    user1.username = String::from("Domino Zelenak");

    println!("user1 name is {}",user1.username);

    // generate users with fn
    //

    let user2 = build_user(String::from("matko@gail.com"),String::from("Matko"));
    let user3 = build_user(String::from("user@gmail.com"),String::from("Patko"));
    let user4 = build_user(String::from("user@gmail.com"),String::from("Zlatk"));
    let user5 = build_user(String::from("user@gmail.com"),String::from("Vratk"));
    let user6 = build_user(String::from("user@gmail.com"),String::from("Radko"));
    println!("{}\n{}\n{}\n{}\n{}",user2.username,user3.username,user4.username,user5.username,user6.username);

    // STRUCT UPDATE SYNTAX
    //

    let user7 = User { 
        // update syntax -> ..struct_instance -> copy
        // all remaining values from other instance
        email: String::from("novy@gmail.com"),
        ..user1 // this line must be last (syntax)
        // WARNING -> we moved data (variable: username is String)
        // from user1 to user7
        // user1 is no longer valid
    };
    println!("user7 = {:?}",user7.username);

    // TOUPLE STRUCTS -> structs that looks similar to touples
    //

    {
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(1,2,3);
        let origin_point = Color(0,0,0);
        
        println!("black values {} {} {}",black.0,black.1,black.2);

        let Color(a,b,c) = black;
        println!("a={} b={} c={}",a,b,c);
        
    }

    // Unit-like structs -> empty structs

    struct AlwaysEqual; // empty (unit-like) struct
    {
        let subject = AlwaysEqual;
    }

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // the same as username:username -> field init shorthand
        email,
        sign_in_count: 1,
    }
}
