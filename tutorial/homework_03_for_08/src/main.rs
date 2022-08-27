use std::collections::HashMap;
use std::io;
use std::cmp::Ordering;
enum Choice {
    Show,
    End,
    Command
}

// HELP?!: 
// https://stackoverflow.com/questions/3458689/how-to-move-screen-without-moving-cursor-in-vim
// https://stackoverflow.com/questions/29378295/how-can-i-force-a-value-to-be-moved-from-an-inner-scope-to-an-outer-instead-of-b



fn main() {

    let mut department_map: HashMap<String,Vec<String>> = HashMap::new();

    println!{"Welcome in CLI to manage your employees: "}

    'user_input: loop{

        // take use input
        let mut buffer_user_input = String::new();
        io::stdin()
            .read_line(&mut buffer_user_input)
            .expect("Failed to read stdin");
       
        let first_word:Vec<String> = get_first_word(buffer_user_input.clone());
        
        
        // evaluate first word of input string
        let mychoice: Choice = match &first_word {
        
            w if &w[0].cmp(&"Show".to_string()) == &Ordering::Equal => Choice::Show,
            w if &w[0].cmp(&"End".to_string()) == &Ordering::Equal => Choice::End,
            _ => Choice::Command
    
        };

        // based on evaluated input string perfrom some action
        match mychoice {
            Choice::Show => {
                println!{"Select department"};
                let mut depart = String::new();
                io::stdin()
                    .read_line(&mut depart)
                    .expect("Failed to read stdin");
                println!{"..> {}",depart};
                let depart = depart.trim();
                list_department(depart, &mut department_map);
            },


            Choice::End => {
                println!{"End"};
                break 'user_input;
            },

            // store tuple to Command V
            Choice::Command =>{

                let entry = department_map.entry(first_word[3].to_string()).or_insert(Vec::new());
                entry.push(first_word[1].clone());
            }
        }

    }

    println!{"HashMap: {:?}",department_map};
}


fn get_first_word(s: String) -> Vec<String> {
    let mut my_vec:Vec<String> = Vec::new();
    for word in s.split_whitespace(){
        my_vec.push(word.to_string());
    }
    my_vec
}


fn list_department(d: &str, map: &mut HashMap<String,Vec<String>>){

    match map.get_mut(d){
        Some(v) => 
        {
            //sorting case insensitive:  
            v.sort_by(|a,b | a.to_lowercase().cmp(&b.to_lowercase())); 
            println!{"Sorted vector {:?}",v};
        },
        _ => println!("nothing")
    };
}
