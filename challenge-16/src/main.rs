use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let string_to_reverse = args.get(1);
    if string_to_reverse.unwrap_or(&"".to_owned()).chars().count() == 0 {
        println!("Please Enter an argument string!")
    } else {
        let reversed_string: String = string_to_reverse.unwrap().chars().rev().collect();
        println!("{}", reversed_string);
    }
}
