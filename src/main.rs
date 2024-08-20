mod email;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Utilisation: {} <email>", args[0]);
        process::exit(1);
    }

    let email = &args[1];

    if !email::is_lowercase(email) {
        println!("'{}' n'est pas entièrement en minuscule.", email);
        process::exit(1);
    }

    if email::is_valid_email(email) {
        println!("'{}' est une adresse mail valide.", email);
    } else {
        println!("'{}' n'est pas une adresse mail valide.", email);
    }
}
