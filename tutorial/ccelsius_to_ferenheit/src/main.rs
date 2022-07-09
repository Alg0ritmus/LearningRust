use std::io;
fn main() {

    println!("Enter the value if Celsius:");
    loop{
        let mut temp_in_c = String::new();
        io::stdin()
            .read_line(&mut temp_in_c)
            .expect("some problems wit stdin");
        let temp_in_c: f64 = match temp_in_c.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,

        };


    let temp_in_f: f64 = temp_in_c * 1.8 + 32.0;
    println!("{temp_in_c} Celsius is {temp_in_f} Ferenheit");
    }
}
