use std::io;
fn main() {
    loop {
        println!("Zadaj cislo:");
        let mut my_num = String::new();
        io::stdin()
            .read_line(&mut my_num)
            .expect("Smth goes wrong with stdin");

        let my_num: u32 = match my_num.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        
        let my_num_sec: u32 = my_num;
        let my_num: u32 = fib(my_num);
        println!("this is my num {my_num}");
        let my_num: u32 = fin_nonrec(my_num_sec);
        println!("this is my num {my_num}");

    }


}


fn fib(num: u32) -> u32 {
    if num == 0  || num == 1{
        return 1;
    }
    fib(num-1)+fib(num-2)
    
}

fn fin_nonrec (num: u32) -> u32 {
   let mut array: [u32;2] = [1,1];
   let mut temp: u32;
   if num == 0 || num == 1{
    return 1;
   }
    for _ in 0..(num-1){
        temp = array[0]+array[1];
        array[0] = array[1];
        array[1] = temp;

    }
    array[1]
}
