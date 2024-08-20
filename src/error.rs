use std::fmt;

// Définition d'un type d'erreur personnalisé
#[derive(Debug)]
pub enum EmailError {
    InvalidFormat,
    NotLowercase,
    NoInternetConnection,
    DnsError,
    InvalidMxRecords,
}

// Implémentation du trait fmt::Display pour notre type d'erreur
impl fmt::Display for EmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            EmailError::InvalidFormat => write!(f, "L'adresse email a un format invalide."),
            EmailError::NotLowercase => write!(f, "L'adresse email n'est pas en minuscules."),
            EmailError::NoInternetConnection => {
                write!(f, "Aucune connexion Internet n'a été détectée.")
            }
            EmailError::DnsError => write!(f, "Erreur DNS lors de la résolution du domaine."),
            EmailError::InvalidMxRecords => {
                write!(f, "Le domaine n'a pas d'enregistrements MX valides.")
            }
        }
    }
}

// Implémentation du trait std::error::Error pour notre type d'erreur
impl std::error::Error for EmailError {}
