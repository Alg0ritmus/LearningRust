fn main() {
    // create vector to store value (like linked-list), that
    // can take any type, but type of all elements in Vec must
    // be the same!!!
    
    // with annotation for values that I want to store (i32)
    let v: Vec<i32> = Vec::new();
    
    // or u can create vec using macro and give it some values
    // so compiler know what type of values we want to use in Vec
    let mut v = vec![1,2,3];
    
    // Add to Vec
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);



    println!("Vector v contanis: {:?}",v);

    // get element from Vec

    // Vec is implemented on heap so u need to use references

    let third: &i32 = &v[2];
    println!("Third element from Vec v: {}",third);

    // check if there is such element in vec using Option type
    //
    // Option type implementation
    //
    // enum Option<T>{
    //      Some(T),
    //      None
    // }
    
    let third: Option<&i32> = v.get(2);
    match third{
        Some(third) => println!("Value on 3rd place in Vec v is {}",third),
        None => println!("There is no element on 3rd place in Vec v")

    }


    // Try to access vlue from out of range index

    let v = vec![1,2,3,4,5];
    
    // This gives us error:
    // let does_not_exists = &v[100];
    // let does_not_exists = v.get(100);


    // Watch out, alway keep in mind ownership and borrowing rules
    // like: There can't be immutable and also mutable reference to value
    // in the same scope

    // This gives us error: 
    let mut v = vec![1,2,3,4,5];
    let first = &v[0]; // immutable
    v.push(6); // here v gets updated -> mutable borrow

    //here we call borowed value that got updated so it's no longer immutable
    //but mutable again

    // println!("first element in vec v is: {}",first);

    // But this is working
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];
    println!("first element in vec v is: {}",first);
    v.push(6);
    let first = &v[0];
    println!("first element in vec v is: {}",first);


    // looping through Vec

    let mut v = vec![1,2,3,4,5];
    for i in &v{
        println!("value:{}",i);
    }

    // mutable looping

    for i in &mut v{
        *i+=50;
        println!("value:{}",i);
    }
   

    // Vec can hold values of same types
    // but we can band the rules somethimes B-)
    // using enums
    
    enum MyType{
        MyInt(i32),
        MyFloat(f64),
        MyText(String),
    }

    let Vec_w_different_types = vec![
        MyType::MyInt(6),
        MyType::MyFloat(3.14),
        MyType::MyText(String::from("Date"))

    ];


    // Play around w String and string slices
    // watching ownership

    let mut s1  = String::from("foo");
    let s2 = "bar";
    let s2 =  String::from(s2);
    let sx =  String::from("baaars");
    
    let s3;
    {
    
    s3 = sx;
    }

    // println!("sx is {}",sx);
    s1.push_str(&s2); // eqivalent to s1.push('char');
    println!("word: {},| s2 is {}",s1,s3);
    

    // CONCATENATION

    let s4 = s1+&s2+&s3; // s1 is moved, s2 and s3 are references, bc. + operator
    // follows signature like this -> fn add (self,s:&str) ->String {...}
    // note that s1 is no longer valid after + operation due to moving
    println!("s4: {}",s4);

    let s1 =  String::from("tic");
    let s2 =  String::from("tac");
    let s3 =  String::from("toe");

    // print using format! macro
    
    let s4 = format!("{}-{}-{}",s1,s2,s3); // works like println! but returns value and does not take ownership
    println!("s4: {}",s4);




    // INDEXING String

    let hello = "Здравствуйте";
    let answer = &hello[0..2];
    println!("first letter {}",answer);
    

    // Iterating over strings
    
    // getting characters
    for c in hello.chars() {
        println!("{}",c);
    }


    // geting bytes
    for c in hello.bytes() {
        println!("{}",c);
    }



    // HASHMAP
    
    use std::collections::HashMap;

    // create HashMap
    
    let mut scores = HashMap::new();
    // add to HashMap<K,V> K-key | V-value
    // Key and Values can have any types, but it must be consistent
    // throughout the HashMap
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Red"),12);
    // Accessing values form HashMap
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    let score_resloved = match score {
        Some(score) => score,
        None => &0
    };
    println!("score of {} team is {}",team_name,score_resloved);

    // Ownership of HashMaps
    // for types that impl. copy trait (like i32) its simply copied
    // for e.g. String, it takes the ownership of it

    let field_name = String::from("Fav car");
    let field_val = String::from("Maserati");

    let mut map = HashMap::new();
    

    //this is not valid!!!
    {
        // here hashmap takes ownership of those vals
        // map.insert(field_name,field_val); 
        // println!("This is not valid! field_name {} ",field_name);
    }

    // but this is valid
    // using references (pointers)
    map.insert(&field_name,&field_val); 
    println!("This is not valid! field_name {} ",field_name);



    // Updating a HashMap
    // You can do multiple things like:
    
    // Overwrite value 
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    let value_score = scores.get(&"Blue".to_string());   
    let vs = match value_score{
        Some(v) => v,
        _ => &0
    };
    println!("score of Blue Team: {:?}",vs);

    // overwritten

    scores.insert(String::from("Blue"),30);
    let value_score = scores.get(&"Blue".to_string());   
    let vs = match value_score{
        Some(v) => v,
        _ => &0
    };
    println!("score of Blue Team: {:?}",vs);


    // Adding a Key and Value Only If a Key Isn’t Present

    let mut map = HashMap::new();

    // get entry, if entry doesn't exist create one and insert value
    map.entry(String::from("Blue")).or_insert(50); 
    map.entry(String::from("Green")).or_insert(10);
    // get entry, if exists, just return entry (no change will happend)
    map.entry(String::from("Blue")).or_insert(11);

    println!("Whole hashmap: {:?}",map);


    // Updating a Value Based on the Old Value
    
    // count word apperence in text
    
    let text = "My car is super, fast and the name is car car Maserati car";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Total amount of word apperence is {:?}",map);

    // Looping throuht HashMap

    for (K,V) in map.iter(){
        println!("Key: {} | Value: {}",K,V);
    }
}
