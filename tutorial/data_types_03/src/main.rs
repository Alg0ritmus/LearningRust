#[allow(dead_code)]  // disable unused var. warnings

// constatnts can be globally decalred, cannot change, always immutable,
// cannot be defined at
// runtime, so u cannot calculate them as a product of function at runtime ...
// convention: UPPERCASE_WITH_UNDERSCORES and comes with type (e.g. u32)
const MY_CONSTANT: u32 = 60*60;
fn main() {
    // variables are immutable by default
    let mut x = 5;
    println!("My number x: {}",x);
    x = 6;
    println!("My number x: {} \n",x);

    // manipulating w const

    println!("This is my const {}",MY_CONSTANT);
    let y = MY_CONSTANT + 999;
    println!("This is my const {} \n",y);

    // shadowing, reuse var using let statement

    let shadow_me = 5;
    println!("Variable before shadowing: {}",shadow_me);
    let shadow_me = shadow_me + 1;
    println!("Variable after shadowing: {}",shadow_me);
    {
        let shadow_me = shadow_me * 2;
        println!("Variable in inner scope, shadowing (current value *2): {}",shadow_me);
    }
    println!("Variable after inner scope,shadowing: {} \n",shadow_me);

    // shadowing, reusing var and changing types

    let spaces = "sss";
    println!("Here spaces if an string: {}",spaces);
    let spaces = spaces.len();
    println!("Here spaces if an inteeger, that express number of chars in str: {}",spaces);



    // 1) DATA TYPES - scalars

    //FLOAT & INT

    let round_down_automatically: u32 = 12/5;
    println!("vysledok 12/5 (round down) {}",round_down_automatically);

    // if u want to get float value after division,
    // (not round down) u must pervorm float/float,
    // otherwise an error will occure

    let round_down_automatically: f32 = 12.0/5.0;
    println!("vysledok 12/5 (round down) {}",round_down_automatically);

    // BOLEAN - just 1 byte in size (u can definitely save some mem space)

    let t = true;
    let f = false;

    let boolean: bool = true;

    // CHAR - u can set emoji as char type
    // char type uses single quotes '' instead of double quote ("")
    // '' - denote char
    // "" - denote string
    
    let c = 'z';
    let cc: char = 'z';


    // 2) DATA TYPES - compound types
    
    // Touples - for gouping multiple types together
    
    // with type annotation

    let tup: (i32, f64, u8) = (500, 6.4, 1);

   // without type annotation

    let tup2 = (500, 6.4, 1);
    
    // get values from touples
    
    // 1) patter matching destructure

    let (x, y, z) = tup2;
    println!("print x value {}",x);

    // 2) using dot method

    let  first_from_touple = tup2.0;
    println!("printing first valu from touple {}",first_from_touple);
    
    // array - data in stack (not heap), same type,
    // fixed number of element unlike Vector
    // vector can shrink and grow in size

    let my_array = [1, 2, 3, 4, 5]; 
   
    // annotation of array 5 values of i32 type

    let my_array2: [i32;5] = [1,2,3,4,5];

    // init array with same value
    // e.g. 5x of value 0

    let init_array_w_0 = [0; 5];
    println!("init array w 0 {:?}",init_array_w_0);
    
    // FUNCTIONS - 1 funtions are declarated outside of main fn.
    // here I only call functions
    another_function(55,'g');
    let addition_number = add(5,1);
    println!("this is after adds {addition_number}");
    // note: in Rust we have expression and statements
    // expressions - return some values
    // statements - not returning values
    // EXPRESSIONS: calling - macros, functions, curlly bracets {}, if-else
    // STATEMENTS: binding values using 'let', declaring functions...
    // !!! U can't assaign statement to statement like this: let a = (let b = 6);
    // but this is valid:
    let bind_a = {
        let b = 5;
        b
    };
    println!("binded a: {bind_a}");


    // CONTROL FLOW - if / while /for
    
    let my_number = 3;

    if my_number < 5 {
        println!("My number {my_number} is less than 5");
    }
    else if my_number > 5 {
        println!("my number {my_number} is greater than 5");
    }
    else {
        println!("My number {my_number} is equal to 5");
    }


    // LOOPS FOR WHILE
    

    // loop - forever until u stop program (ctrl-c || break)
    
    // loop {println!("again!") }
    

    // LOOP LABELING | BREAKING NOT INNER LOOP
    // loop labeling uses one single quotes '
    // note: even if we break outer loop, the loop inside continues...

    let mut count = 0;
    let mut remaining = 10;
    'counting_up: loop {
        println!("count = {count}");
        remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -=1;

        }
        count+=1;
    }
    println!("END count = {count}");
    println!("remaining = {remaining}");

    // WHILE loop
    //

    let a = [10,20,30,40];
    let mut index = 0;
    while index < 4 {
        println!("the value of {index}-th item in array is: {}",a[index]);
        index += 1;
    }

    // FOR loop
    //

    for element in a {
        println!("The element: {element} is in array a");
    }

    for num in (1..4).rev() { //rev() just reverse the order, inclusive..exclusiv
        println!("printing value {num}");
    }
    println!("KONIEC");
}


fn another_function(x: i32, unit_label: char) {
    println!("Im another function! {} {}",x, unit_label);
}

// RETURNING VALUE FROM FUNCT

fn add(x: i32, y: i32)-> i32 {
    let z: i32 = 1;
    x+y+z

}
// If we want to return value at last line of fn
// do not use semicolon at the end of last line
// or u can return with keyword 'return'
