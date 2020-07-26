use std::env;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    let string_to_reverse = args.get(1);
    if string_to_reverse.unwrap_or(&"".to_owned()).chars().count() == 0 {
        println!("Please Enter an argument string!")
    } else {
        let reversed_string: String = string_to_reverse.unwrap().chars().rev().collect();
        let time_elasped: f64 = start_time.elapsed().as_nanos() as f64 / 1000000000 as f64;
        println!("{}", reversed_string);
        println!("Time taken in seconds: {}", time_elasped)
    }
}
