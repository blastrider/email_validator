use regex::Regex;
use crate::error::EmailError;

pub fn is_lowercase(email: &str) -> Result<(), EmailError> {
    if email == email.to_lowercase() {
        Ok(())
    } else {
        Err(EmailError::NotLowercase)
    }
}

pub fn is_valid_email(email: &str) -> Result<(), EmailError> {
    let email_regex = Regex::new(
        r"(?i)^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$",
    ).unwrap();

    if email_regex.is_match(email) {
        Ok(())
    } else {
        Err(EmailError::InvalidFormat)
    }
}