use std::collections::HashMap;
fn main(){
    ////////////////////////////////////////
    // HashMap (idx Vec->Vec->Vec-> String)
    ////////////////////////////////////////

    let mut map: HashMap<i32,Vec<Vec<Vec<String>>>> = HashMap::new();

    map.insert(0,vec![vec![vec!["item_to_access".to_string()]]]);

    println!{"My HM: {:?}",map};
    
    println!{"Accessing intended val from HM: {:?}",map.get(&0).unwrap()[0][0][0]};

    ////////////////////////////////////////
    // How to add vals to most inner vec?
    ////////////////////////////////////////

    match map.get_mut(&mut 0) { // get_mut() instead of get() to get mutable ref
    // get_mut here https://doc.rust-lang.org/std/vec/struct.Vec.html#method.get_mut
       Some(v) => {
            v[0][0].push("lol".to_string());
            println!{"Test adding, {:?}",v[0][0]};

        },
        _ => println!{"Nothing happend"}
    }
    println!{"My HM: {:?}",map};

    
    ////////////////////////////////////////
    // Update Val -//-
    ////////////////////////////////////////


    match map.get_mut(&mut 0) { 
       Some(v) => {
            v[0][0][0] = "first_val_changed".to_string();
            println!{"Test update, {:?}",v[0][0]};

        },
        _ => println!{"Nothing happend"}
    }
    println!{"My HM: {:?}",map};


    ////////////////////////////////////////
    // Delete Val -//-
    ////////////////////////////////////////


    match map.get_mut(&mut 0) { 
       Some(v) => {
            v[0][0].remove(0);
            println!{"Test del, {:?}",v[0][0]};

        },
        _ => println!{"Nothing happend"}
    }
    println!{"My HM: {:?}",map};



//end of main()
}
