pub fn validate_username(value: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let trimmed = value.trim();
    if trimmed.is_empty() {
        errors.push("must not be empty".to_string());
    }
    if trimmed.len() < 3 {
        errors.push("must be at least 3 characters".to_string());
    }
    if trimmed.len() > 32 {
        errors.push("must be at most 32 characters".to_string());
    }
    if trimmed.chars().any(|c| c.is_whitespace()) {
        errors.push("must not contain spaces".to_string());
    }
    if !trimmed.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        errors.push("contains invalid characters".to_string());
    }
    errors
}

pub fn validate_team_name(value: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let trimmed = value.trim();
    if trimmed.is_empty() {
        errors.push("must not be empty".to_string());
    }
    if trimmed.len() < 3 {
        errors.push("must be at least 3 characters".to_string());
    }
    if trimmed.len() > 32 {
        errors.push("must be at most 32 characters".to_string());
    }
    if trimmed.chars().any(|c| c.is_whitespace()) {
        errors.push("must not contain spaces".to_string());
    }
    if !trimmed.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        errors.push("contains invalid characters".to_string());
    }
    errors
}
