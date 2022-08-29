use std::collections::HashMap;
fn main(){

    
    //########
    //# Minimal example of updatind hashmap from inner scope
    //########

    // let mut map: HashMap<i32,Vec<&str>> = HashMap::new();

    // {
    //     for i in 0..10{

    //         let inner_scope_word = "from_inner_scope";
    //         map.insert(i,vec![&inner_scope_word]);
    //     }
    // }
    // println!{"mymap: {:?}",map};

    
    //########
    //# Try with &str instead of String (working with references + function + lifetime)
    //# WORKING
    //########


    let mut map: HashMap<String,Vec<&str>> = HashMap::new();
    {
        for i in 0..10{

            let inner_scope_word = "from_inner_scope_w_str";
            let idx: String = i.to_string();
            add_to_hashmap(idx,inner_scope_word,&mut map);
        }
    }
    println!{"mymap: {:?}",map};
}


fn add_to_hashmap<'a>(idx: String, word: &'a str, hm: &mut HashMap<String,Vec<&'a str>>){
    
    let mut my_vec = Vec::new();
    my_vec.push(word);

    hm.insert(idx,my_vec);
}


//########
//# Try with &str instead of String (working with references + function + lifetime)
//# NOT WORKING !!!!!!!!!!!!!
//########

// let mut map: HashMap<&str,Vec<&str>> = HashMap::new();
// {
//     for i in 0..10{
// 
//         let inner_scope_word = "from_inner_scope_w_str";
//         // You cant pass reference to function of temporary variable,
//         // bcs. then we would create dangling pointer, since we would refer
//         // to memory which is freed after scope ends
//         let idx: &str = &*i.to_string(); 
//         add_to_hashmap(idx,inner_scope_word,&mut map);
//     }
// }
// println!{"mymap: {:?}",map};
// }
// 
// 
// 
// fn add_to_hashmap<'a>(idx: &'a str, word: &'a str, hm: &mut HashMap<&'a str<&'a str>>){
// 
//     let mut my_vec = Vec::new();
//     my_vec.push(word);
// 
//     hm.insert(idx,my_vec);
// }
