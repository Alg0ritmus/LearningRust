fn main() { 
    // HW_01: for chapter 08
    //
    // Given a list of integers, use a vector and return the median 
    // (when sorted, the value in the middle position)
    // and mode (the value that occurs most often; 
    // a hash map will be helpful here) of the list.
    
    let mut list_of_int = vec![1,3,2,2,5,6,4,8,6,6,1,2,2,3];
    
    // GET MEDIAN

    let mut median = 0;
    let mut mode = 0;
    // sort list 
    //println!("before sort {:?}",list_of_int);
    list_of_int.sort();
    println!{"after sort {:?}",list_of_int};

    if list_of_int.len()%2==0 {
        let half = list_of_int.len()/2;
        median = list_of_int[half-1]+list_of_int[half];
        median /= 2;
    }
    else{
        median = list_of_int[list_of_int.len()/2];
    }
    println!{"median of vec is {}",median};


    // GET MODE
    
    // import HashMap
    use std::collections::HashMap; 
    
    // init HashMap
    let mut map = HashMap::new();

    // fill up HashMap, if value occure more than once +=1, else 0
    for value in list_of_int.iter(){
        let item_in_map = map.entry(value).or_insert(0);
        *item_in_map +=1;
    }

    println!{"map: {:?}",map};

    // get the highest value from HashMap
    
    let mut mode_ref = &&0; // using new var, because I dont know how to get K as value
    for (k,v) in map.iter(){
        if v>&mode{
            println!{"biggest key so far is {} with value of {}",k,v};
            mode_ref = k;
            mode = *v;
        }    
    }
    println!{"mode of vec is {}",mode_ref};
    
    
}
