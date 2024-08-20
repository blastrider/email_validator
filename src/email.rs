use regex::Regex;

pub fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(
        r"(?i)^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$",
    ).unwrap();

    email_regex.is_match(email)
}
