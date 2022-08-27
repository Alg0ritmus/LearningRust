fn main() {
    
    let text = "Здравствуйте";

    let a = pig_latin(&text); 

    println!("Hello, {:?}",a);
}

fn pig_latin(s: &str) -> String {

    let vowel = "aoeiu";
    // s is reference so its not moved here 
    // (moving by assigment is not happening)
    let first_letter = match s.chars().next(){
        Some(val) => val,
        _ => ' '
    };

    for l in vowel.chars(){

        if first_letter == l{
            return String::from(s)+&String::from("-hay");
        }
    }



    let cut_first_letter = s.replace(first_letter," ");
    let cut_first_letter = cut_first_letter.trim();
    let mut ending = String::from("-");
    ending.push(first_letter);
    ending.push_str("ay");
    String::from(cut_first_letter)+&ending
}
