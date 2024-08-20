use crate::error::EmailError;
use std::net::TcpStream;

pub fn check_internet_connection() -> Result<(), EmailError> {
    // Essayer de se connecter à Google (port 80 pour HTTP)
    if TcpStream::connect("google.com:80").is_ok() {
        Ok(())
    } else {
        Err(EmailError::NoInternetConnection)
    }
}
