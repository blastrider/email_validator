use crate::error::EmailError;

pub fn validate_local_part(local_part: &str) -> Result<(), EmailError> {
    // Vérifiez la longueur du local-part
    if local_part.is_empty() || local_part.len() > 64 {
        return Err(EmailError::InvalidFormat);
    }

    // Vérifiez que le local-part ne commence ni ne se termine par un point
    if local_part.starts_with('.') || local_part.ends_with('.') {
        return Err(EmailError::InvalidFormat);
    }

    // Vérifiez qu'il n'y a pas de points consécutifs
    if local_part.contains("..") {
        return Err(EmailError::InvalidFormat);
    }

    // Vérifiez que le local-part contient uniquement des caractères valides
    let valid_chars = |c: char| {
        c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '%' || c == '+' || c == '-'
    };

    if !local_part.chars().all(valid_chars) {
        return Err(EmailError::InvalidFormat);
    }

    Ok(())
}
