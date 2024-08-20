use crate::{email::local_part::validate_local_part, error::EmailError};

pub struct EmailParts {
    pub username: String,
    pub domain: String,
}

pub fn parse_email(email: &str) -> Result<EmailParts, EmailError> {
    let parts: Vec<&str> = email.split('@').collect();

    if parts.len() != 2 {
        return Err(EmailError::InvalidFormat);
    }

    // Validation du local-part
    validate_local_part(parts[0])?;

    Ok(EmailParts {
        username: parts[0].to_string(),
        domain: parts[1].to_string(),
    })
}
