use crate::error::EmailError;
use std::collections::HashSet;

pub struct TempEmailValidator {
    temp_domains: HashSet<&'static str>,
}

impl TempEmailValidator {
    pub fn new() -> Self {
        let mut temp_domains = HashSet::new();

        // Ajoutez ici une liste de domaines connus pour les e-mails temporaires
        temp_domains.insert("10minutemail.com");
        temp_domains.insert("guerrillamail.com");
        temp_domains.insert("mailinator.com");
        temp_domains.insert("yopmail.com");
        temp_domains.insert("temp-mail.org");
        // Ajoutez autant de domaines que nécessaire

        TempEmailValidator { temp_domains }
    }

    pub fn is_temporary_email(&self, domain: &str) -> Result<(), EmailError> {
        if self.temp_domains.contains(domain) {
            Err(EmailError::TemporaryEmailAddress)
        } else {
            Ok(())
        }
    }
}
