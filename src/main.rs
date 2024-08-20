mod email;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <email>", args[0]);
        process::exit(1);
    }

    let email = &args[1];
    
    if email::is_valid_email(email) {
        println!("'{}' is a valid email address.", email);
    } else {
        println!("'{}' is not a valid email address.", email);
    }
}