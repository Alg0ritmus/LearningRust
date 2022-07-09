use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number!");

    let random_num = rand::thread_rng().gen_range(1..10);

    loop {

        let mut guess = String::new();
        println!("Please input your guess");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failded to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_num) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            }

        }
    }



}
