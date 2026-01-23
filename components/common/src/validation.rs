// Common validation utilities

pub fn validate_email(email: &str) -> bool {
    // Basic email validation
    email.contains('@') && email.contains('.')
}

pub fn validate_phone(phone: &str) -> bool {
    // Basic phone validation
    phone
        .chars()
        .all(|c| c.is_ascii_digit() || c == '+' || c == '-' || c == ' ' || c == '(' || c == ')')
}
