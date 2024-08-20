use crate::error::EmailError;
use crate::network::check_internet_connection;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

pub fn validate_mx_records(domain: &str) -> Result<(), EmailError> {
    // Vérifiez la connexion Internet
    check_internet_connection()?;

    // Configurer le résolveur DNS
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())
        .map_err(|_| EmailError::DnsError)?;

    // Interroger les enregistrements MX pour le domaine
    let response = resolver
        .mx_lookup(domain)
        .map_err(|_| EmailError::InvalidMxRecords)?;

    // Vérifiez s'il y a des enregistrements MX valides
    if response.iter().next().is_none() {
        return Err(EmailError::InvalidMxRecords);
    }

    Ok(())
}
