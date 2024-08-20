use crate::error::EmailError;

pub fn validate_special_chars(local_part: &str) -> Result<(), EmailError> {
    // Caractères spéciaux autorisés dans le local-part
    let special_chars = "!#$%&'*+-/=?^_`{|}~.";

    // Vérifiez que les caractères spéciaux sont bien utilisés
    for (i, c) in local_part.chars().enumerate() {
        if special_chars.contains(c) {
            // Un caractère spécial ne doit pas être le premier ou le dernier caractère
            if i == 0 || i == local_part.len() - 1 {
                return Err(EmailError::InvalidFormat);
            }
            // Vérifiez que les caractères spéciaux ne sont pas répétés de manière incorrecte (ex. "..")
            if i > 0 && local_part.chars().nth(i - 1) == Some(c) {
                return Err(EmailError::InvalidFormat);
            }
        }
    }

    Ok(())
}
