use std::env;
fn main() {
    let args: Vec<_> = env::args().collect();
    if &args[1] == "add" {
        println!("The sum is: {}", &args[2].parse::<f64>().unwrap() + &args[3].parse::<f64>().unwrap());
    }
}
