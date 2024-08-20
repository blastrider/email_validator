mod email;
mod error;

use crate::email::parser_module::parse_email;
use email::general_format::is_lowercase;
use email::general_format::is_valid_email;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <email>", args[0]);
        process::exit(1);
    }

    let email = &args[1];

    if let Err(e) = is_lowercase(email) {
        eprintln!("{}", e);
        process::exit(1);
    }

    if let Err(e) = is_valid_email(email) {
        eprintln!("{}", e);
        process::exit(1);
    }

    match parse_email(email) {
        Ok(parts) => {
            println!("'{}' is a valid email address.", email);
            println!("Username: {}", parts.username);
            println!("Domain: {}", parts.domain);
        }
        Err(e) => {
            eprintln!("Failed to parse email: {}", e);
            process::exit(1);
        }
    }
}
