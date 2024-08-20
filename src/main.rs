mod dns;
mod email;
mod error;
mod network;

use dns::mx_records::validate_mx_records;
use email::general_format::is_lowercase;
use email::general_format::is_valid_email;
use email::temp_mails::TempEmailValidator;

use crate::email::parser_module::parse_email;

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
            println!("'{}' est une adresse mail valide.", email);
            println!("Nom d'utilisateur: {}", parts.username);
            println!("Domaine: {}", parts.domain);

            // Valider les enregistrements MX du domaine
            if let Err(e) = validate_mx_records(&parts.domain) {
                eprintln!("Failed to validate MX records: {}", e);
                process::exit(1);
            }
            println!("The domain '{}' has valid MX records.", parts.domain);

            // Vérifier si l'adresse e-mail est temporaire
            let validator = TempEmailValidator::new();
            if let Err(e) = validator.is_temporary_email(&parts.domain) {
                eprintln!("Invalid email: {}", e);
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Erreur de parsing de l'adresse mail: {}", e);
            process::exit(1);
        }
    }
}
