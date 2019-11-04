use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if &args[1] == "add" {
        let slice = &args[2..args.len()];
        let mut sum: f64 = 0.0;
        for i in slice {
            sum = sum + i.parse::<f64>().unwrap();
        }
        println!("The sum is: {}", sum);
    } else if &args[1] == "subtract" {
        let slice = &args[3..args.len()];
        let mut diffrence: f64 = args[2].parse::<f64>().unwrap();
        for i in slice {
            diffrence = diffrence - i.parse::<f64>().unwrap();
        }
        println!("The sum is: {}", diffrence);
    }
}
