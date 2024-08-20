use crate::error::EmailError;
use regex::Regex;

pub fn validate_domain(domain: &str) -> Result<(), EmailError> {
    // Vérifiez la longueur du domaine
    if domain.is_empty() || domain.len() > 255 {
        return Err(EmailError::InvalidFormat);
    }

    // Vérifiez que le domaine ne commence ni ne se termine par un point
    if domain.starts_with('.') || domain.ends_with('.') {
        return Err(EmailError::InvalidFormat);
    }

    // Vérifiez qu'il n'y a pas de points consécutifs
    if domain.contains("..") {
        return Err(EmailError::InvalidFormat);
    }

    // Utilisation d'une expression régulière pour valider le domaine
    let domain_regex = Regex::new(r"^(?i)[a-z0-9.-]+\.[a-z]{2,}$").unwrap();

    if !domain_regex.is_match(domain) {
        return Err(EmailError::InvalidFormat);
    }

    Ok(())
}
