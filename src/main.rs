mod email;
mod error;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <email>", args[0]);
        process::exit(1);
    }

    let email = &args[1];

    if let Err(e) = email::is_lowercase(email) {
        eprintln!("{}", e);
        process::exit(1);
    }

    if let Err(e) = email::is_valid_email(email) {
        eprintln!("{}", e);
        process::exit(1);
    }

    println!("'{}' est une adresse mail valide.", email);
}
